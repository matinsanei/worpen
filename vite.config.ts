import path from 'path';
import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig(({ mode }) => {
    const env = loadEnv(mode, '.', '');
    return {
      server: {
        // Dev UI port. The Rust binary serves API+UI on :3000 in production;
        // in dev, Vite runs here and proxies API calls to the backend.
        port: Number(env.VITE_PORT) || Number(process.env.PORT) || 5173,
        host: '0.0.0.0',
        proxy: {
          '/api': 'http://127.0.0.1:3000',
          '/mcp': 'http://127.0.0.1:3000',
          '/health': 'http://127.0.0.1:3000',
          '/swagger-ui': 'http://127.0.0.1:3000',
          '/api-docs': 'http://127.0.0.1:3000',
        },
      },
      plugins: [react()],
      define: {
        'process.env.API_KEY': JSON.stringify(env.GEMINI_API_KEY),
        'process.env.GEMINI_API_KEY': JSON.stringify(env.GEMINI_API_KEY)
      },
      resolve: {
        alias: {
          '@': path.resolve(__dirname, '.'),
        }
      }
    };
});
