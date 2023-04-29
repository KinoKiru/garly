<template>
    <div class="group !relative">
        <slot name="title" />

        <div
            v-if="defaultMenu"
            class="dropdown-hover">
            <div
                class="dropdown-menu nes-container"
                :class="{menuClasses: true, 'with-title is-centered' : menuTitle !== '', 'is-dark top-14': isDark} ">
                <div
                    v-if="menuTitle"
                    class="title">
                    <p>{{ menuTitle }}</p>
                </div>
                <slot
                    name="body" />
            </div>
        </div>
        <slot
            v-else
            name="body" />
    </div>
</template>

<script setup lang="ts">
    import { isDark } from '~/logic';

    withDefaults(defineProps<{
        defaultMenu?: boolean;
        menuClasses?: string;
        menuTitle?: string | null;
    }>(), {
        defaultMenu: true,
        menuClasses: '',
        menuTitle: null,
    });

</script>

<style scoped lang="postcss">
   .dropdown-hover {
        @apply group-hover:flex hover:flex hidden;
        @apply absolute top-0 z-10 min-w-30 right-0;
    }
    .dropdown-menu {
        @apply mt-14 flex flex-col;
    }
</style>
