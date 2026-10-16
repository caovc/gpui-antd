// Import generated wasm-bindgen bindings. Never substitute a DOM component
// gallery here: the visible controls are real Rust gpui-antd views.
const status = document.getElementById('status');
const params = new URLSearchParams(location.search);
const component = params.get('component') || 'Button';
const dark = params.get('dark') === '1';
document.body.classList.toggle('dark', dark);

const touchOnly = matchMedia('(hover: none) and (pointer: coarse)').matches;
// GPUI Web uses a hidden HTML input to route keyboard and IME to the canvas.
// Avoid opening the virtual keyboard until a user intentionally interacts.
const observer = new MutationObserver(records => {
  if (!touchOnly) return;
  for (const record of records) for (const item of record.addedNodes) {
    if (item instanceof HTMLInputElement && item.parentElement === document.body) {
      item.readOnly = true;
      item.setAttribute('inputmode', 'none');
      if (document.activeElement === item) item.blur();
    }
  }
});
observer.observe(document.body, {childList: true});

try {
  const wasm = await import('./wasm/gpui_antd_gallery.js');
  await wasm.default();
  status.querySelector('span').textContent = 'Initializing GPU surface…';
  wasm.run(component, dark);
  // run_embedded starts asynchronous graphics initialization. A return from
  // wasm.run is NOT proof that rendering succeeded. Keep the status visible
  // until a canvas has been inserted and has real dimensions.
  const waitForCanvas = async () => {
    for (let i = 0; i < 600; i++) {
      const canvas = document.querySelector('canvas');
      if (canvas && canvas.width > 0 && canvas.height > 0) {
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
        status.remove();
        return;
      }
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    throw new Error('GPUI did not initialize a drawing surface. Try a browser with WebGPU or WebGL2.');
  };
  await waitForCanvas();
} catch (error) {
  console.error('GPUI WASM startup failed:', error);
  status.classList.add('error');
  status.innerHTML = '<strong>Unable to start GPUI preview</strong><p></p>';
  status.querySelector('p').textContent = String(error?.message || error);
}
