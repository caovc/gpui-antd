import test from 'node:test';
import assert from 'node:assert/strict';
import { createIconPreview, ICON_PREVIEW_TIMEOUT_MS } from '../src/icon-preview.js';

const base = 'https://example.test/gpui-antd/';
const route = name => base + 'demo.html?component=Icon%3A' + name + '&dark=0';
function withoutAttempt(url) {
  const value = new URL(url);
  value.searchParams.delete('previewRequest');
  return value.href;
}

function fixture() {
  let now = 0, next = 0;
  const timers = new Map();
  const messages = new Set();
  const blurs = new Set();
  const focusListeners = new Set();
  const handlers = { message: messages, blur: blurs };
  let observer;
  const owner = {
    location: { origin: 'https://example.test' },
    addEventListener: (name, handler) => handlers[name].add(handler),
    removeEventListener: (name, handler) => handlers[name].delete(handler),
    queueMicrotask: handler => handler(),
    setTimeout: (handler, ms) => { const id = ++next; timers.set(id, { at: now + ms, handler }); return id; },
    clearTimeout: id => timers.delete(id),
    MutationObserver: class {
      constructor(callback) { this.callback = callback; observer = this; }
      observe() { this.connected = true; }
      disconnect() { this.connected = false; }
    },
  };
  const requests = [];
  const frame = Object.assign(new EventTarget(), {
    ownerDocument: { defaultView: owner, baseURI: base, documentElement: {}, body: {},
      addEventListener: (_, handler) => focusListeners.add(handler),
      removeEventListener: (_, handler) => focusListeners.delete(handler) },
    dataset: {}, isConnected: true, contentWindow: {}, contentDocument: null,
    attributes: {}, setAttribute(name, value) { this.attributes[name] = value; },
  });
  Object.defineProperty(frame, 'src', {
    get: () => requests.at(-1), set: value => requests.push(value),
  });
  const retry = Object.assign(new EventTarget(), { hidden: true });
  function focus(element) {
    frame.ownerDocument.activeElement = element;
    for (const handler of focusListeners) handler({ target: element });
  }
  const status = { textContent: '', isConnected: true, focus() { focus(this); } };
  const controller = createIconPreview(frame, status, retry);
  function currentAttempt(url) {
    return frame.src && withoutAttempt(frame.src) === url ? frame.src : url;
  }
  function commit(url, { width = 128, height = 128, startup = false } = {}) {
    frame.contentDocument = { URL: currentAttempt(url), querySelector: selector =>
      selector === 'canvas' ? { width, height } : selector === '#status' && startup ? {} : null };
    frame.dispatchEvent(new Event('load'));
  }
  function message(url, { type = 'gpui-antd:ready', source = frame.contentWindow, origin = owner.location.origin } = {}) {
    const sender = new URL(currentAttempt(url));
    for (const handler of messages) handler({ source, origin,
      data: { type, component: sender.searchParams.get('component'),
        previewRequest: sender.searchParams.get('previewRequest'), url: sender.href } });
  }
  function tick(ms) {
    now += ms;
    for (const [id, item] of [...timers]) {
      if (item.at <= now && timers.delete(id)) item.handler();
    }
  }
  return { controller, frame, retry, status, requests, timers, messages, blurs, focusListeners, focus,
    commit, message, tick, detach() { frame.isConnected = false; observer.callback(); },
    observer: () => observer };
}

test('requested src and load alone never mark a GPUI preview ready', () => {
  const f = fixture(), a = route('SearchOutlined');
  assert.equal(f.controller.request(a), true);
  assert.equal(f.frame.dataset.previewState, 'loading');
  assert.equal(f.frame.inert, true);
  assert.equal(f.frame.attributes['aria-hidden'], 'true');
  assert.equal(f.frame.attributes['aria-busy'], 'true');
  assert.equal(f.frame.loading, 'eager');
  f.commit(a);
  assert.equal(f.frame.dataset.previewState, 'loading');
  f.message(a);
  assert.equal(f.frame.dataset.previewState, 'ready');
  assert.equal(f.frame.inert, false);
  assert.equal(f.frame.attributes['aria-hidden'], 'false');
  assert.equal(f.frame.attributes['aria-busy'], 'false');
  assert.equal(f.timers.size, 0);
  assert.equal(f.controller.request(a), false);
  assert.equal(f.controller.request(a, { retry: true }), false);
  assert.equal(f.requests.length, 1);
  f.controller.destroy();
});

test('ready messages require current frame, origin, component, exact URL and painted canvas', () => {
  const f = fixture(), a = route('CiTwoTone');
  f.controller.request(a);
  f.commit(a);
  f.message(a, { source: {} });
  f.message(a, { origin: 'https://other.test' });
  f.message(route('SearchOutlined'));
  assert.equal(f.frame.dataset.previewState, 'loading');
  for (const options of [{ width: 0 }, { height: 0 }, { startup: true }]) {
    f.commit(a, options); f.message(a);
    assert.equal(f.frame.dataset.previewState, 'loading');
  }
  // A different color/size route shares the component name but is stale.
  f.commit(a + '&size=96'); f.message(a);
  assert.equal(f.frame.dataset.previewState, 'loading');
  f.commit(a); f.message(a);
  assert.equal(f.frame.dataset.previewState, 'ready');
  f.controller.destroy();
});

test('rapid selection ignores previous readiness and expired previous deadlines', () => {
  const f = fixture(), a = route('SearchOutlined'), b = route('CiTwoTone');
  f.controller.request(a);
  const oldDeadline = [...f.timers.values()][0].handler;
  f.controller.request(b);
  f.commit(a); f.message(a); oldDeadline();
  assert.equal(f.frame.dataset.previewState, 'loading');
  assert.equal(withoutAttempt(f.requests.at(-1)), b);
  f.commit(b); f.message(b);
  oldDeadline(); f.tick(ICON_PREVIEW_TIMEOUT_MS);
  assert.equal(f.frame.dataset.previewState, 'ready');
  f.controller.destroy();
});

test('reselecting the desired icon retries cancellation despite an unchanged desired route', () => {
  const f = fixture(), a = route('SearchOutlined'), b = route('CiTwoTone');
  f.controller.request(a); f.commit(a); f.message(a);
  f.controller.request(b); // cancellation leaves the actual document at a
  assert.equal(withoutAttempt(f.frame.src), b);
  assert.equal(withoutAttempt(f.frame.contentDocument.URL), a);
  assert.equal(f.frame.inert, true);
  f.message(a);
  assert.equal(f.frame.dataset.previewState, 'loading');
  assert.equal(f.controller.request(b), false); // duplicate settings event
  assert.equal(f.controller.request(b, { retry: true }), true);
  assert.deepEqual(f.requests.map(withoutAttempt), [a, b, b]);
  assert.notEqual(f.requests[1], f.requests[2]);
  f.commit(b); f.message(b);
  assert.equal(f.frame.dataset.previewState, 'ready');
  f.controller.destroy();
});

test('readiness deadline offers retry and keeps the stale canvas inert', () => {
  const f = fixture(), a = route('CiTwoTone');
  f.controller.request(a);
  f.tick(ICON_PREVIEW_TIMEOUT_MS - 1);
  assert.equal(f.frame.dataset.previewState, 'loading');
  f.tick(1);
  assert.equal(f.frame.dataset.previewState, 'error');
  assert.match(f.status.textContent, /taking too long/);
  assert.equal(f.frame.inert, true);
  assert.equal(f.retry.hidden, false);
  f.frame.ownerDocument.activeElement = f.retry;
  f.retry.dispatchEvent(new Event('click'));
  assert.equal(f.frame.ownerDocument.activeElement, f.status);
  assert.equal(f.frame.dataset.previewState, 'loading');
  assert.equal(f.retry.hidden, true);
  assert.deepEqual(f.requests.map(withoutAttempt), [a, a]);
  assert.notEqual(f.requests[0], f.requests[1]);
  f.commit(a); f.message(a);
  assert.equal(f.frame.dataset.previewState, 'ready');
  f.controller.destroy();
});

test('current renderer error offers recovery; stale errors cannot replace the latest state', () => {
  const f = fixture(), a = route('SearchOutlined'), b = route('CiTwoTone');
  f.controller.request(a); f.commit(a);
  f.controller.request(b);
  f.message(a, { type: 'gpui-antd:error' });
  assert.equal(f.frame.dataset.previewState, 'loading');
  f.commit(b, { startup: true }); f.message(b, { type: 'gpui-antd:error' });
  assert.equal(f.frame.dataset.previewState, 'error');
  assert.equal(f.retry.hidden, false);
  f.controller.request(b); f.commit(b); f.message(b);
  assert.equal(f.frame.dataset.previewState, 'ready');
  f.message(a, { type: 'gpui-antd:error' });
  assert.equal(f.frame.dataset.previewState, 'ready');
  f.controller.destroy();
});

test('late valid paint resolves a timeout without accepting any previous route', () => {
  const f = fixture(), a = route('CiTwoTone');
  f.controller.request(a); f.tick(ICON_PREVIEW_TIMEOUT_MS);
  f.commit(route('SearchOutlined')); f.message(route('SearchOutlined'));
  assert.equal(f.frame.dataset.previewState, 'error');
  f.frame.ownerDocument.activeElement = f.retry;
  f.commit(a); f.message(a);
  assert.equal(f.frame.dataset.previewState, 'ready');
  assert.equal(f.retry.hidden, true);
  assert.equal(f.frame.ownerDocument.activeElement, f.status);
  f.controller.destroy();
});

test('an error queued by an earlier same-route retry cannot fail the new attempt', () => {
  const f = fixture(), a = route('CiTwoTone');
  f.controller.request(a);
  const previous = f.frame.src;
  f.commit(a, { startup: true });
  f.controller.request(a, { retry: true });
  f.commit(a, { startup: true });
  f.message(previous, { type: 'gpui-antd:error' });
  assert.equal(f.frame.dataset.previewState, 'loading');
  f.commit(a);
  f.message(previous);
  assert.equal(f.frame.dataset.previewState, 'loading');
  f.message(a);
  assert.equal(f.frame.dataset.previewState, 'ready');
  f.controller.destroy();
});

test('route removal cancels deadlines and releases listeners; cross-origin routes are rejected', () => {
  const f = fixture();
  assert.throws(() => f.controller.request('https://other.test/demo.html'), /same-origin/);
  f.controller.request(route('CiTwoTone'));
  f.detach();
  assert.equal(f.messages.size, 0);
  assert.equal(f.blurs.size, 0);
  assert.equal(f.focusListeners.size, 0);
  assert.equal(f.timers.size, 0);
  assert.equal(f.observer().connected, false);
  assert.equal(f.controller.request(route('SearchOutlined')), false);
  f.retry.dispatchEvent(new Event('click'));
  f.tick(ICON_PREVIEW_TIMEOUT_MS);
  assert.equal(f.requests.length, 1);
  f.controller.destroy();
});

test('initialization preserves the latest host focus without trapping the ready preview', () => {
  const f = fixture(), a = route('CiTwoTone');
  const first = { isConnected: true, focus() { f.focus(this); } };
  const latest = { isConnected: true, focus() { f.focus(this); } };
  f.focus(first);
  f.controller.request(a);
  f.focus(latest);
  f.focus(f.frame); // browser moves focus into GPUI's initializing IME mirror
  for (const handler of f.blurs) handler();
  assert.equal(f.frame.ownerDocument.activeElement, latest);
  f.focus(f.frame);
  f.commit(a); f.message(a);
  assert.equal(f.frame.ownerDocument.activeElement, latest);
  // Once ready, intentionally entering the preview must remain possible.
  f.focus(f.frame);
  for (const handler of f.blurs) handler();
  f.message(a);
  assert.equal(f.frame.ownerDocument.activeElement, f.frame);
  f.controller.destroy();
});

test('an adjacent GPUI example cannot steal host focus while it initializes', () => {
  const f = fixture(), a = route('CiTwoTone');
  const control = { isConnected: true, focus() { f.focus(this); } };
  let initializing = true;
  const example = { isConnected: true, tagName: 'IFRAME', matches: () => true,
    contentDocument: { URL: base + 'demo.html?component=Icons',
      querySelector: () => initializing ? {} : null }, focus() { f.focus(this); } };
  f.focus(control);
  f.controller.request(a); f.commit(a); f.message(a);
  f.focus(example);
  for (const handler of f.blurs) handler();
  assert.equal(f.frame.ownerDocument.activeElement, control);
  initializing = false;
  f.focus(example);
  for (const handler of f.blurs) handler();
  assert.equal(f.frame.ownerDocument.activeElement, example);
  example.contentDocument.querySelector = () => ({ classList: { contains: () => true } });
  for (const handler of f.blurs) handler();
  assert.equal(f.frame.ownerDocument.activeElement, example);
  f.controller.destroy();
});
