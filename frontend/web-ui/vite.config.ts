// frontend/web-ui/vite.config.ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [
    vue(),
  ],

	server: {
    fs: {
      // Permite servir archivos fuera de la raíz si fuera necesario
      allow: ['..']
    }
  },
  assetsInclude: ['**/*.wasm']
  // Si no quieres usar plugins, asegúrate de que el binario se sirva correctamente
})
