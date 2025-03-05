import { defineConfig } from "vitest/config";
import { resolve } from 'path'

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	plugins: [],
	root: 'src',
  resolve: {
    alias: [
      { find: "@Renderer", replacement: resolve(__dirname, "./src/renderer") },
      { find: "@Assets", replacement: resolve(__dirname, "./src/static") },
      { find: "@Types", replacement: resolve(__dirname, "./src/renderer/types") },
    ]
  },
  test: {
		coverage: {
      provider: "v8",
			exclude: ['**/*.{spec,test,unit,accept,integrate,system,perf,stress}.{ts,tsx}']
		},
		include: [
			// '**/*.{system,perf,stress}.ts',
			'**/*.{spec,test,unit,accept,integrate}.{ts,tsx}'
		],
    globals: true,
    environment: 'jsdom',
    setupFiles: './tests/setup.ts',
	},
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  // to access the Tauri environment variables set by the CLI with information about the current target
  envPrefix: ['VITE_', 'TAURI_PLATFORM', 'TAURI_ARCH', 'TAURI_FAMILY', 'TAURI_PLATFORM_VERSION', 'TAURI_PLATFORM_TYPE', 'TAURI_DEBUG'],
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  define: {
    global: "window",
  }
});
