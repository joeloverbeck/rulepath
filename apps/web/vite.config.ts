import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  base: "./",
  plugins: [react()],
  build: {
    // Deliberate budget for the presentation bundle. The shell statically
    // imports every catalog board, so the chunk grows with the catalog;
    // raising the advisory limit makes a crossing a real signal, not noise.
    chunkSizeWarningLimit: 600,
    rollupOptions: {
      output: {
        // Peel rarely-changing React runtime into its own cached chunk,
        // keeping the app chunk below the warning ceiling.
        manualChunks: {
          vendor: ["react", "react-dom"],
        },
      },
    },
  },
});
