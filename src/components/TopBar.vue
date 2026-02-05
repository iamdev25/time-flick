<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  isRunning: boolean;
  currentTime: string;
}>();

const statusText = computed(() => props.isRunning ? 'Tracking Time...' : 'Timer Paused');
const statusColor = computed(() => props.isRunning ? 'text-green-500' : 'text-gray-400');
</script>

<template>
  <header class="bg-white shadow px-6 py-4 flex justify-between items-center z-10">
    <h2 class="text-xl font-semibold text-gray-800">
      Dashboard
    </h2>
    
    <div class="flex items-center space-x-4">
      <div class="flex flex-col items-end">
         <span class="text-xs font-semibold uppercase tracking-wide text-gray-500">Status</span>
         <span class="text-sm font-medium flex items-center gap-2">
            <span class="relative flex h-3 w-3" v-if="isRunning">
              <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
              <span class="relative inline-flex rounded-full h-3 w-3 bg-green-500"></span>
            </span>
            <span :class="statusColor">{{ statusText }}</span>
         </span>
      </div>
      
      <div v-if="isRunning" class="bg-gray-100 px-3 py-1 rounded text-lg font-mono font-bold text-gray-700 border border-gray-200">
        {{ currentTime }}
      </div>
    </div>
  </header>
</template>
