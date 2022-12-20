import { createRouter, createWebHistory } from "vue-router";

import Main from './views/Main.vue';
import Processes from './views/Processes.vue';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', name: 'main', component: Main },
        { path: '/processes/:id', name: "processes", component: Processes, props: true },
    ]
})

export default router;