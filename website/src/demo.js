import { WITHDRAWN_COMPONENTS } from './component-names.js';

// Import generated wasm-bindgen bindings. Never substitute a DOM component
// gallery here: the visible controls are real Rust gpui-antd views.
const status = document.getElementById('status');
const params = new URLSearchParams(location.search);
const component = params.get('component') || 'Button';
const dark = params.get('dark') === '1';
const previewRequest = params.get('previewRequest');
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
  const name = component.split('|')[0];
  if (WITHDRAWN_COMPONENTS.includes(name)) {
    throw new Error(name + ' is unavailable: its placeholder implementation was withdrawn. See the component support guide.');
  }
  const wasm = await import('./wasm/gpui_antd_gallery.js');
  await wasm.default();
  status.querySelector('span').textContent = 'Initializing GPU surface…';
  wasm.run(component, dark);
  // run_embedded starts asynchronous graphics initialization. A return from
  // wasm.run is NOT proof that rendering succeeded. Keep the status visible
  // until both the canvas and GPUI Web's hidden IME textarea exist. The
  // canvas can precede asynchronous GPU setup and ImeMirror::new, which
  // focuses that textarea; acknowledging earlier ends host focus protection
  // before keyboard initialization has actually completed.
  const waitForCanvas = async () => {
    for (let i = 0; i < 600; i++) {
      const canvas = document.querySelector('canvas');
      const keyboard = document.querySelector('body > textarea');
      if (canvas && canvas.width > 0 && canvas.height > 0 && keyboard) {
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
        status.remove();
        if(window.parent!==window)window.parent.postMessage({
          type:'gpui-antd:ready',component,previewRequest,url:location.href
        },location.origin);
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
  if(window.parent!==window)window.parent.postMessage({
    type:'gpui-antd:error',component,previewRequest,url:location.href,error:String(error?.message||error)
  },location.origin);
}
