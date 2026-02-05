<script setup lang="ts">
import { useRoute } from 'vue-router';
import { LayoutDashboard, BarChart3, Settings } from 'lucide-vue-next';

const route = useRoute();

const navigation = [
  { name: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
  { name: 'Reports', href: '/reports', icon: BarChart3 },
  { name: 'Settings', href: '/settings', icon: Settings },
];

const isActive = (path: string) => route.path === path;
</script>

<template>
  <div class="w-64 bg-dark-surface text-gray-300 flex flex-col h-full border-r border-gray-700">
    <div class="p-6 border-b border-gray-700">
      <h1 class="text-2xl font-bold text-white tracking-wider">Time Flick</h1>
    </div>
    
    <nav class="flex-1 px-4 py-6 space-y-2">
      <router-link
        v-for="item in navigation"
        :key="item.name"
        :to="item.href"
        class="flex items-center px-4 py-3 text-sm font-medium rounded-md transition-colors duration-150 group"
        :class="isActive(item.href) ? 'bg-primary text-white' : 'hover:bg-gray-700 hover:text-white'"
      >
        <component 
          :is="item.icon" 
          class="mr-3 h-6 w-6 flex-shrink-0" 
          :class="isActive(item.href) ? 'text-white' : 'text-gray-400 group-hover:text-white'" 
          aria-hidden="true" 
        />
        {{ item.name }}
      </router-link>
    </nav>

    <div class="p-4 border-t border-gray-700">
       <div class="flex items-center">
          <div class="h-8 w-8 rounded-full bg-gray-500 flex items-center justify-center text-white font-bold">
            JA
          </div>
          <div class="ml-3">
            <p class="text-sm font-medium text-white">Julfikar Ali</p>
            <p class="text-xs text-gray-400">Freelancer</p>
          </div>
       </div>
    </div>
  </div>
</template>
