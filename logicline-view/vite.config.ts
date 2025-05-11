import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const lib_name = "logicline-view";

export default defineConfig({
  plugins: [react()],
  build: {
    rollupOptions: {
      external: ["react", "react-dom", "react/jsx-runtime"],
      output: {
        globals: {
          react: "React",
          "react-dom": "ReactDOM"
        }
      }
    },
    lib: {
      entry: "./src/lib.mts",
      name: lib_name,
      fileName: (format) => `${lib_name}.${format}.js`
    }
  }
});
