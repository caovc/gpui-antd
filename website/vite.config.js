import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import { resolve } from 'node:path';

export default defineConfig({
  base: '/gpui-antd/',
  plugins: [wasm()],
  build: {
    target: 'esnext',
    rollupOptions: {
      input: {
        docs: resolve(import.meta.dirname, 'index.html'),
        demo: resolve(import.meta.dirname, 'demo.html'),
      },
      onwarn(warning, warn) {
        if (warning.code === 'EVAL' && warning.id?.includes('/src/wasm/gpui_antd_gallery.js')) return;
        warn(warning);
      },
    },
  },
  server: {
    host: '0.0.0.0',
    port: 3000,
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
  },
});
