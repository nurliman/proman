import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import tailwindcss from "tailwindcss"; // @ts-ignore
import postcssFlexbugsFixes from "postcss-flexbugs-fixes";
import postcssPresetEnv from "postcss-preset-env";

const serverHost = process.env.SERVER_HOST || "127.0.0.1";
const serverPort = process.env.SERVER_PORT || 3000;
const serverUrl = `http://${serverHost}:${serverPort}`;

export default defineConfig((config) => ({
  plugins: [sveltekit()],
  css: {
    postcss: {
      plugins: [
        tailwindcss(),
        postcssFlexbugsFixes(),
        postcssPresetEnv({
          autoprefixer: {
            flexbox: "no-2009",
            grid: "autoplace",
          },
          stage: 3,
          features: {
            "custom-properties": false,
          },
        }),
      ],
    },
  },
  server: {
    proxy:
      config.command === "serve"
        ? {
            "/api": {
              target: serverUrl,
              changeOrigin: true,
              rewrite: (path) => path.replace(/^\/api/, ""),
            },
          }
        : undefined,
  },
}));
