import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import * as fs from 'fs'
import * as path from 'path'

// Plugin to copy info-panel.html to dist directory
const copyInfoPanelHtml = () => ({
  name: 'copy-info-panel-html',
  writeBundle() {
    const srcPath = path.join(__dirname, 'src-tauri/info-panel.html')
    const destPath = path.join(__dirname, 'dist/info-panel.html')
    fs.copyFileSync(srcPath, destPath)
  }
})

export default defineConfig(async () => ({
  plugins: [vue(), copyInfoPanelHtml()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
}))
