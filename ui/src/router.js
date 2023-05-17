import { createRouter, createWebHashHistory } from "vue-router";
import VmList from "./views/VmList.vue";
import NetworkList from "./views/NetworkList.vue";
import StorageList from "./views/StorageList.vue";

export default createRouter({
  // 4. Provide the history implementation to use. We are using the hash history for simplicity here.
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: "/machine" },
    { path: '/machine', component: VmList },
    { path: '/network', component: NetworkList },
    { path: '/storage', component: StorageList },
  ],
});