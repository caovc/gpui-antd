// Navigation belongs to the docs shell; the frame still paints real Rust GPUI.
// A requested iframe.src is not evidence that navigation committed or painted.
export const ICON_PREVIEW_TIMEOUT_MS = 60_000;

export function createIconPreview(frame, status, retry) {
  const document = frame.ownerDocument;
  const owner = document.defaultView;
  let target = '';
  let phase = 'idle';
  let attempt = 0;
  let timer;
  let disposed = false;

  function currentDocument() {
    try { return frame.contentDocument; } catch { return null; }
  }

  function painted() {
    const doc = currentDocument();
    const canvas = doc?.querySelector('canvas');
    return doc?.URL === target && canvas?.width > 0 && canvas?.height > 0 &&
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
    attempt++;
    show('loading', 'Loading icon preview…');
    armDeadline();
    // An active inspector request must start even while the old frame is
    // hidden. Reassigning the same URL intentionally retries cancellation.
    frame.loading = 'eager';
    frame.src = href;
    return true;
  }

  function onMessage(event) {
    if (disposed || !target || !frame.isConnected) return;
    if (event.origin !== owner.location.origin || event.source !== frame.contentWindow) return;
    if (event.data?.component !== new URL(target).searchParams.get('component')) return;
    // WindowProxy identity survives iframe navigations. Also verify the
    // actual loaded document, including all color/size/theme parameters.
    if (currentDocument()?.URL !== target) return;
    if (event.data?.type === 'gpui-antd:ready' && painted()) {
      clearDeadline();
      show('ready', 'Preview ready');
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
    frame.removeEventListener('load', onLoad);
    retry.removeEventListener('click', onRetry);
    observer.disconnect();
  }

  const observer = new owner.MutationObserver(() => {
    if (!frame.isConnected) destroy();
  });
  observer.observe(document.documentElement, { childList: true, subtree: true });
  owner.addEventListener('message', onMessage);
  frame.addEventListener('load', onLoad);
  retry.addEventListener('click', onRetry);
  show('idle', 'Loading icon preview…');
  return { request, destroy };
}
