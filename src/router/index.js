import { createRouter, createWebHashHistory } from "vue-router"
import HomeScreen from "../views/HomeScreen.vue";
import StaticScreen from "../views/StaticScreen.vue";
import Cam2Focus from "../views/Cam2Focus.vue";
import CamFocus from "../views/CamFocus.vue";
import mockWS from "../views/mockWS.vue";


const routes = [
  { path: '/', component: HomeScreen },
  { path: '/StaticScreen', component: StaticScreen },
  { path: '/2', component: Cam2Focus },
  { path: '/CamFocus/:id', component: CamFocus },
  { path: '/test', component: mockWS },
]

const router = createRouter({
  // Provide the history implementation to use. We
  // are using the hash history for simplicity here.
  history: createWebHashHistory(),
  routes, // short for `routes: routes`
})

export default router
