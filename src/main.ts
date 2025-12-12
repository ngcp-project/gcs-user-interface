import { createApp } from "vue";
import "./main.css";
import App from "./App.vue";
import router from "./router";
import { createPinia } from 'pinia'
import { establishTaurpcConnection } from "./lib/StoresSync";

createApp(App).use(router).use(createPinia()).mount("#app");
establishTaurpcConnection(); //This line must come after the Line 9 to properly instantiate Pinia stores.
