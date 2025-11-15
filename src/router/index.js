import { createRouter, createWebHistory } from 'vue-router'
import Page1 from '../pages/recording.vue'
import Page2 from '../pages/configuration.vue'
import Page3 from '../pages/theme.vue'
import Page4 from '../pages/examples.vue'


const routes = [
  { path: '/', redirect: '/Recording' },
  { path: '/Recording', name: 'Recording', component: Page1, meta: { title: 'Recording Management' } },
  { path: '/Configuration', name: 'Configuration', component: Page2, meta: { title: 'Configuration' } },
  { path: '/Theme', name: 'Theme', component: Page3, meta: { title: 'Theme' } },
  { path: '/Examples', name: 'Examples', component: Page4, meta: { title: 'Examples' } },
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
