import { fileURLToPath, URL } from "node:url";

import { resolve } from "path";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        irina: resolve(__dirname, "flashcards.html"),
        colors: resolve(__dirname, "colors.html"),
        login: resolve(__dirname, "login.html"),
        register: resolve(__dirname, "register.html"),
        vitamin: resolve(__dirname, "vitamincalculator.html"),
      },
    },
  },
});
