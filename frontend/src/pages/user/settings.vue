<template>
    <h2>
        settings
    </h2>

    <button
        type="button"
        class="nes-btn w-min"
        :class="isDark ? '' : '!bg-[#0f0f0f98]'"
        @click="setInverse">
        {{ isDark ? 'licht' : 'donker' }}
    </button>
</template>

<script setup lang="ts">
    import { isDark, toggleDark } from '~/logic';

    function setInverse() {
        toggleDark(!isDark.value);
        localStorage.setItem('vueuse-color-scheme', isDark.value ? 'dark' : 'light');
    }
</script>

<style scoped>
 button {
        border-image-repeat: unset;
        background-clip: unset;
    }
</style>
