import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import * as path from 'path'

export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, 'index.html'),
        'info-panel': path.resolve(__dirname, 'windows/info-panel.html'),
        'settings': path.resolve(__dirname, 'windows/settings.html'),
        'todo-panel': path.resolve(__dirname, 'windows/todo-panel.html')
      }
    }
  }
}))
