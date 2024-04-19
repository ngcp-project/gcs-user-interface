import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";
import testDisplayComponents from "./testDisplayComponents.vue";

createApp(App).use(router).mount("#app");