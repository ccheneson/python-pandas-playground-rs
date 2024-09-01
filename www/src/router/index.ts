import { createRouter, createWebHistory } from 'vue-router'
import ApiCodeForm from '@/components/ApiCodeForm.vue'
import ApiForm from '@/components/ApiForm.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Home'
    },
    {
      path: '/code',
      name: 'code',
      component: ApiCodeForm
    },
    {
      path: '/execute',
      name: 'execute',
      component: ApiForm
    }
  ]
})

export default router
