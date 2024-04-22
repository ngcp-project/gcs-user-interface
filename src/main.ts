import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";
import { initializeWSConnections } from "./webSocket";

// initialize 4 websocket connections for all 4 vehicles once app stars
initializeWSConnections();
createApp(App).use(router).mount("#app");