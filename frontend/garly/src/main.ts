import { createApp } from 'vue';
import generatedRoutes from 'virtual:generated-pages';
import { setupLayouts } from 'virtual:generated-layouts';
import { createRouter, createWebHistory } from 'vue-router';
import './styles/main.css';
import App from './App.vue';
import 'virtual:uno.css';
import 'nes.css/css/nes.min.css';

const app = createApp(App);

const routes = setupLayouts(generatedRoutes);
const router = createRouter({ history: createWebHistory(), routes });
app.use(router);

app.config.errorHandler = (error) => {
    // eslint-disable-next-line no-console
    console.error(error);
    return false;
};

Object.values(import.meta.glob('./modules/*.ts', { eager: true, import: 'install' })).forEach((install: any) => install?.({ app, router, routes }));

app.mount('#app');
