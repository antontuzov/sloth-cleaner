import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  // Vite options tailored for Tauri development
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  // Prevent vite from obscuring rust errors
  logLevel: process.env.TAURI_ENV_DEBUG ? 'silent' : 'info',
  // Handle Tauri API module (only available at runtime)
  optimizeDeps: {
    exclude: ['@tauri-apps/api'],
  },
  build: {
    rollupOptions: {
      external: ['@tauri-apps/api/core'],
      output: {
        globals: {
          '@tauri-apps/api/core': '__TAURI_API__',
        },
      },
    },
  },
})
