import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { gestaltsPlugins } from '@gestalts/vite';

/** `tauri android dev` sets this to an address the phone reaches this machine at. */
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  server: {
    port: 5174,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: 'ws', host, port: 5175 } : undefined,
  },
  // the app's IPC, from the client and the barcode scanner alike: one copy
  resolve: { dedupe: ['@tauri-apps/api'] },
  plugins: [gestaltsPlugins(), react(), tailwindcss()],
  base: './',
});
