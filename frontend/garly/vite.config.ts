/* eslint-disable @typescript-eslint/naming-convention */
import { defineConfig } from 'vite';
import path from 'path';
import UnoCSS from 'unocss/vite';
import Vue from '@vitejs/plugin-vue';
import AutoImport from 'unplugin-auto-import/vite';
import Pages from 'vite-plugin-pages';
import Layouts from 'vite-plugin-vue-layouts';
import Components from 'unplugin-vue-components/vite';
import VueI18n from '@intlify/unplugin-vue-i18n/vite';

// https://vitejs.dev/config/
export default defineConfig(() => ({
    resolve: {
        alias: {
            '~/': `${path.resolve(__dirname, 'src')}/`,
            '@/': `${path.resolve(__dirname, 'src', 'components')}/`,
        },
    },
    plugins: [
        Vue(),
        UnoCSS(),
        Pages({
            extensions: ['vue'],
        }),
        Layouts(),

        Components({
            // Allow auto load markdown components under `./src/components/`
            extensions: ['vue'],
            // Allow auto import and register components for md
            // Include: [/\.vue$/, /\.vue\?vue/],
            directoryAsNamespace: true,
            globalNamespaces: ['partial', 'modal'],

            dts: 'src/components.d.ts',
        }),

        AutoImport({
            imports: ['vue', 'vue-router', 'vue-i18n', '@vueuse/head', '@vueuse/core'],
            dts: 'src/auto-imports.d.ts',
        }),

        VueI18n({
            runtimeOnly: true,
            compositionOnly: true,
            include: [path.resolve(__dirname, 'locales/**')],
        }),

    ],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // prevent vite from obscuring rust errors
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
    },
    optimizeDeps: {
        include: ['vue', 'vue-router', '@vueuse/core', '@vueuse/head'],
    },
    // to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
    // Tauri supports es2021
        target: process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
        // don't minify for debug builds
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        // produce sourcemaps for debug builds
        sourcemap: !!process.env.TAURI_DEBUG,
    },
}));
