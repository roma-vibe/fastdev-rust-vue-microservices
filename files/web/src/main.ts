import { createPinia } from 'pinia';
import { createApp } from 'vue';
import App from './App.vue';
import { router } from './app/router.ts';
import { i18n } from './shared/i18n.ts';
import './shared/theme.ts';
import './styles/main.css';

createApp(App).use(createPinia()).use(i18n).use(router).mount('#app');
