import { defineConfig } from "vite"

export default defineConfig({
  build: {
    assetsDir: 'js/root',
    manifest: 'manifest.json',
    rollupOptions: {
      external: [
        'single-spa',
      ],
      input: 'src/root-config.js',
      output: {
        format: 'system',
      }
    }
  }
})
