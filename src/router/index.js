import { createRouter, createWebHashHistory } from "vue-router"
import fourCam from "../views/fourCam.vue";
import Cam1Focus from "../views/Cam1Focus.vue";
import Cam2Focus from "../views/Cam2Focus.vue";
import test from "../views/test.vue";


const routes = [
  { path: '/', component: fourCam },
  { path: '/1', component: Cam1Focus },
  { path: '/2', component: Cam2Focus },
  { path: '/test', component: test },
]

const router = createRouter({
  // 4. Provide the history implementation to use. We
  // are using the hash history for simplicity here.
  history: createWebHashHistory(),
  routes, // short for `routes: routes`
})

export default router
