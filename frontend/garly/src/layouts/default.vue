<template>
    <div class="flex flex-col">
        <nav class="nav">
            <p class="mb-0">
                Garly
            </p>
            <div class="actions">
                <div>
                    ss
                </div>
                <div class="nes-field">
                    <input
                        type="search"
                        class="nes-input is-dark h-10">
                </div>
            </div>
        </nav>
        <RouterView class="flex-1" />
    </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="postcss">
    .nav {
        @apply w-full bg-dark-200 text-white h-15 flex items-center justify-between px-3;
    }

    .actions{
        @apply flex justify-evenly;
    }

    input[type="search"]{
        border-image-repeat: unset;
    }
</style>
