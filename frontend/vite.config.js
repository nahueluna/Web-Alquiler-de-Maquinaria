import react from "@vitejs/plugin-react";
import path from "path";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@mui/material": path.resolve(__dirname, "node_modules/@mui/joy"),
    },
  },
});
