import { createRouter, createWebHashHistory } from "vue-router";
import HomeScreen from "../views/HomeScreen.vue";
import StaticScreen from "../views/StaticScreen.vue";

const routes = [
  { path: "/", component: HomeScreen },
  { path: "/StaticScreen", component: StaticScreen }
];

const router = createRouter({
  // Provide the history implementation to use. We
  // are using the hash history for simplicity here.
  history: createWebHashHistory(),
  routes // short for `routes: routes`
});

export default router;
