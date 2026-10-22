// Navigation belongs to the docs shell; the frame still paints real Rust GPUI.
// A requested iframe.src is not evidence that navigation committed or painted.
export const ICON_PREVIEW_TIMEOUT_MS = 60_000;

export function createIconPreview(frame, status, retry) {
  const document = frame.ownerDocument;
  const owner = document.defaultView;
  let target = '';
  let documentUrl = '';
  let phase = 'idle';
  let attempt = 0;
  let timer;
  let disposed = false;
  let focusOwner;

  function rememberFocus(event) {
    const element = event.target;
    if (element && element !== frame && element.tagName !== 'IFRAME' && element !== document.body &&
        element !== document.documentElement && element.isConnected &&
        typeof element.focus === 'function') focusOwner = element;
  }

  function initializingExample(element) {
    if (!element?.matches?.('iframe.wasm-iframe')) return false;
    try {
      const doc = element.contentDocument;
      const startup = doc?.querySelector('#status');
      return doc && new URL(doc.URL).origin === owner.location.origin &&
        Boolean(startup) && !startup.classList?.contains('error');
    } catch { return false; }
  }

  function restoreFocus() {
    if (!focusOwner?.isConnected) return;
    if ([frame, document.body, document.documentElement].includes(document.activeElement) ||
        initializingExample(document.activeElement)) {
      focusOwner.focus({ preventScroll: true });
    }
  }

  function onBlur() {
    // GPUI initializes a hidden IME textarea and focuses it. A microtask
    // observes the resulting active iframe before the next keyboard task.
    owner.queueMicrotask(() => {
      if (!disposed && (phase !== 'ready' || initializingExample(document.activeElement))) restoreFocus();
    });
  }

  function currentDocument() {
    try { return frame.contentDocument; } catch { return null; }
  }

  function painted() {
    const doc = currentDocument();
    const canvas = doc?.querySelector('canvas');
    return doc?.URL === documentUrl && canvas?.width > 0 && canvas?.height > 0 &&
      !doc.querySelector('#status');
  }

  function clearDeadline() {
    owner.clearTimeout(timer);
    timer = undefined;
  }

  function show(next, message) {
    // Retry can disappear after activation or a late successful paint.
    // Preserve a meaningful keyboard target in either case.
    if (next !== 'error' && document.activeElement === retry) {
      status.focus({ preventScroll: true });
    }
    phase = next;
    frame.dataset.previewState = next;
    frame.inert = next !== 'ready';
    frame.setAttribute('aria-busy', String(next === 'loading'));
    frame.setAttribute('aria-hidden', String(next !== 'ready'));
    status.textContent = message;
    retry.hidden = next !== 'error';
  }

  function fail(message) {
    clearDeadline();
    show('error', message);
  }

  function armDeadline() {
    clearDeadline();
    const generation = attempt;
    timer = owner.setTimeout(() => {
      if (disposed || generation !== attempt || phase !== 'loading') return;
      fail('Icon preview is taking too long. Retry preview.');
    }, ICON_PREVIEW_TIMEOUT_MS);
  }

  function request(url, { retry: reselect = false } = {}) {
    if (disposed) return false;
    if (!frame.isConnected) { destroy(); return false; }
    const href = new URL(url, document.baseURI).href;
    if (new URL(href).origin !== owner.location.origin) {
      throw new Error('Icon previews must use a same-origin GPUI route');
    }
    if (href === target) {
      if (phase === 'ready' && painted()) return false;
      if (phase === 'loading' && !reselect) return false;
    }
    target = href;
    rememberFocus({ target: document.activeElement });
    attempt++;
    const navigation = new URL(href);
    navigation.searchParams.set('previewRequest', String(attempt));
    documentUrl = navigation.href;
    show('loading', 'Loading icon preview…');
    armDeadline();
    // An active inspector request must start even while the old frame is
    // hidden. Reassigning the same URL intentionally retries cancellation.
    frame.loading = 'eager';
    frame.src = documentUrl;
    return true;
  }

  function onMessage(event) {
    if (disposed || !target || !frame.isConnected) return;
    if (event.origin !== owner.location.origin || event.source !== frame.contentWindow) return;
    if (event.data?.component !== new URL(target).searchParams.get('component')) return;
    if (event.data?.previewRequest !== String(attempt) || event.data?.url !== documentUrl) return;
    // WindowProxy identity survives iframe navigations. Also verify the
    // actual loaded document, including all color/size/theme parameters.
    if (currentDocument()?.URL !== documentUrl) return;
    if (event.data?.type === 'gpui-antd:ready' && painted()) {
      const initializing = phase !== 'ready';
      clearDeadline();
      show('ready', 'Preview ready');
      if (initializing) restoreFocus();
    } else if (event.data?.type === 'gpui-antd:error' && phase !== 'ready') {
      fail('Icon preview could not be loaded. Retry preview.');
    }
  }

  function onLoad() {
    if (phase === 'ready' && !painted()) {
      show('loading', 'Loading icon preview…');
      armDeadline();
    }
  }

  function onRetry() {
    if (!target) return;
    request(target, { retry: true });
  }

  function destroy() {
    if (disposed) return;
    disposed = true;
    clearDeadline();
    owner.removeEventListener('message', onMessage);
    owner.removeEventListener('blur', onBlur);
    document.removeEventListener('focusin', rememberFocus);
    frame.removeEventListener('load', onLoad);
    retry.removeEventListener('click', onRetry);
    observer.disconnect();
  }

  const observer = new owner.MutationObserver(() => {
    if (!frame.isConnected) destroy();
  });
  observer.observe(document.documentElement, { childList: true, subtree: true });
  owner.addEventListener('message', onMessage);
  owner.addEventListener('blur', onBlur);
  document.addEventListener('focusin', rememberFocus);
  frame.addEventListener('load', onLoad);
  retry.addEventListener('click', onRetry);
  show('idle', 'Loading icon preview…');
  return { request, destroy };
}
