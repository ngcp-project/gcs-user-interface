import { createRouter, createWebHashHistory } from "vue-router"
import fourCam from "../views/fourCam.vue";
import test from "../views/test.vue";


const routes = [
  { path: '/', component: fourCam },
  { path: '/test', component: test },
]

const router = createRouter({
  // 4. Provide the history implementation to use. We
  // are using the hash history for simplicity here.
  history: createWebHashHistory(),
  routes, // short for `routes: routes`
})

export default router
