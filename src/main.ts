import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";
import yuh from "./views/testMapScreen.vue"

createApp(yuh).use(router).mount("#app");
