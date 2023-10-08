import { defineConfig, searchForWorkspaceRoot } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import basicSsl from '@vitejs/plugin-basic-ssl'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte(), basicSsl()],
    server: {
        proxy: {
            "/api": "http://127.0.0.1:8080",
        },
        fs: {
            allow: [searchForWorkspaceRoot(process.cwd())]
        }
    },
});
