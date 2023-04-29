<template>
    <nav class="nav">
        <RouterLink
            to="/"
            class="text-white hover:text-light-900">
            Garly
        </RouterLink>
        <div class="actions">
            <div
                v-for="route, i in routes"
                :key="i"
                class="flex items-center justify-evenly mr-2">
                <Dropdown v-if="route.children">
                    <template #title>
                        <RouterLink
                            :to="route.path"
                            class="hover:text-light-900">
                            {{ route.meta.name }}
                        </RouterLink>
                    </template>
                    <template #body>
                        <template v-for="subroute in route.children">
                            {{ subroute.name }}
                        </template>
                    </template>
                </Dropdown>
                <RouterLink
                    v-else
                    :to="route.path"
                    class="hover:text-light-900">
                    {{ route.meta.name }}
                </RouterLink>
            </div>
            <div class="nes-field">
                <input
                    type="search"
                    :placeholder="t('nav.search')"
                    :class="{'is-dark': isDark}"
                    class="nes-input h-10">
            </div>
            <Dropdown menu-title="Youri">
                <template #title>
                    <div class="i-material-symbols-person text-5xl" />
                </template>
                <template #body>
                    <ul
                        v-for="route in generatedRoutes.filter(route => (route.name as string)?.includes('user'))"
                        :key="route.name"
                        class="px-3">
                        <li class="flex justify-start">
                            <RouterLink
                                :to="route.path">
                                {{ (route.name as string).replace('user-', '') }}
                            </RouterLink>
                        </li>
                    </ul>
                </template>
            </Dropdown>
        </div>
    </nav>
</template>

<script setup lang="ts">
    import generatedRoutes from 'virtual:generated-pages';
    import { RouterLink } from 'vue-router';
    import { isDark } from '~/logic';
    const { t } = useI18n();

    const routes = computed(() => generatedRoutes.map((route: any) => {
        route.meta ??= {};
        if (typeof route.meta.translation_key === 'string') {
            route.meta.name = t(route.meta.translation_key);
        }
        return route;
    }));
</script>

<style scoped lang="postcss">
    .nav {
        @apply bg-indigo-500 text-dark-300 dark:(bg-dark-200 text-white);
        @apply w-full h-15 flex items-center justify-between pl-3;
    }

    .actions{
        @apply flex justify-evenly items-center;
    }

    input[type="search"]{
        border-image-repeat: unset;
        background-clip: unset;
    }

    p {
        @apply mb-0;
    }

</style>
