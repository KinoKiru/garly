<template>
    <div class="flex flex-col items-center justify-items-center justify-center">
        <h1>Welcome to Tauri!</h1>

        <div class="flex">
            <a
                href="https://vitejs.dev"
                target="_blank">
                <img
                    src="/vite.svg"
                    class="w-25"
                    alt="Vite logo">
            </a>
            <a
                href="https://tauri.app"
                target="_blank">
                <img
                    src="/tauri.svg"
                    class="w-25"
                    alt="Tauri logo">
            </a>
            <a
                href="https://vuejs.org/"
                target="_blank">
                <img
                    :src="logo"
                    class="w-25"
                    alt="Tauri logo">
            </a>
        </div>

        <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

        <p>
            Recommended IDE setup:
            <a
                href="https://code.visualstudio.com/"
                target="_blank">VS Code</a>
            +
            <a
                href="https://github.com/johnsoncodehk/volar"
                target="_blank">Volar</a>
            +
            <a
                href="https://github.com/tauri-apps/tauri-vscode"
                target="_blank">Tauri</a>
            +
            <a
                href="https://github.com/rust-lang/rust-analyzer"
                target="_blank">rust-analyzer</a>
        </p>

        <Greet />
    </div>
</template>

<script setup lang="ts">
    // @ts-ignore
    import logo from '~/assets/vue.svg';
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>

<route lang="yaml">
    meta:
        translation_key: 'nav.home'
</route>
