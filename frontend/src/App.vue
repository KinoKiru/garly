<template>
    <div
        v-if="loading"
        class="flex flex-col justify-center align-middle items-center h-full">
        <!-- <CirqllLogo class="h-28 <md:w-52 w-max" /> -->
        <div
            v-if="error"
            class="text-red-500 text-center">
            {{ error }}
        </div>
        <div v-else>
            We're getting ready for you, please wait.
        </div>
    </div>
    <RouterView v-else />
</template>

<script setup lang="ts">
    // https://github.com/vueuse/head
    // You can use this to manipulate the document head in any components,
    useHead({
        title: 'garly',
        meta: [{ name: 'description', content: 'garly' }],
    });

    /*
     * ## Preload all data that is needed (eg. user, subscription and preferences) ##
     */
    const loading = ref(true);
    const error = ref<string>();

    onMounted(() => {
        loading.value = false;

    });
</script>
