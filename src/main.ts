import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./app/App.vue";
import { router } from "./app/router";
import VueVirtualScroller from "vue-virtual-scroller";
import "vue-virtual-scroller/dist/vue-virtual-scroller.css";

const app = createApp(App);

app.config.errorHandler = (err, instance, info) => {
  console.error("Global error boundary caught an error:", err, info);
};

app.use(createPinia());
app.use(router);
app.use(VueVirtualScroller);
app.mount("#app");
