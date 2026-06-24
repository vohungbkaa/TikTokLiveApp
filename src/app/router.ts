import { createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/live-setup' },
    { path: '/live-setup', component: () => import('../features/LiveSetup.vue') },
    { path: '/products', component: () => import('../features/ProductManagement.vue') },
    { path: '/live-console', component: () => import('../features/LiveConsole.vue') },
    { path: '/parser-debug', component: () => import('../features/ParserDebug.vue') },
    { path: '/order-review', component: () => import('../features/OrderReview.vue') },
    { path: '/settings', component: () => import('../features/Settings.vue') }
  ]
});
