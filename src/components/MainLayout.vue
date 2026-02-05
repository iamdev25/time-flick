<script setup lang="ts">
import Sidebar from './Sidebar.vue';
import TopBar from './TopBar.vue';

defineProps<{
  isRunning?: boolean;
  currentTime?: string;
}>();
</script>

<template>
  <div class="flex h-screen bg-gray-50 overflow-hidden">
    <!-- Sidebar -->
    <Sidebar />

    <!-- Main Content -->
    <div class="flex-1 flex flex-col min-w-0 overflow-hidden">
      <TopBar :is-running="isRunning || false" :current-time="currentTime || '00:00:00'" />
      
      <main class="flex-1 overflow-y-auto p-6">
        <slot />
      </main>
    </div>
  </div>
</template>
