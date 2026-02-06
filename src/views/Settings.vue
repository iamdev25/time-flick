<script setup lang="ts">
import MainLayout from '../components/MainLayout.vue';
import { useScreenshotSystem } from '../composables/useScreenshotSystem';
import { ref } from 'vue';
import { Camera, Clock, Monitor, History, Trash2, Check, AlertCircle } from 'lucide-vue-next';

const { isEnabled, intervalMinutes, history, takeScreenshot } = useScreenshotSystem();

const isTaking = ref(false);
const errorMsg = ref<string | null>(null);
const successMsg = ref<string | null>(null);

const intervals = [
  { label: 'Every 5 Minutes', value: 5 },
  { label: 'Every 10 Minutes', value: 10 },
  { label: 'Every 30 Minutes', value: 30 },
];

const handleTakeNow = async () => {
    isTaking.value = true;
    errorMsg.value = null;
    successMsg.value = null;
    try {
        await takeScreenshot();
        successMsg.value = 'Screenshot captured successfully';
        setTimeout(() => successMsg.value = null, 3000);
    } catch (e: any) {
        errorMsg.value = e.toString();
    } finally {
        isTaking.value = false;
    }
};

const clearHistory = () => {
    history.value = [];
    localStorage.removeItem('screenshotHistory');
};

// Helper to convert file path to asset URL if needed, or just display raw path
// Tauri might need specific protocol `boxicons` to show local images or use `convertFileSrc`.
import { convertFileSrc } from '@tauri-apps/api/core';

</script>

<template>
  <MainLayout>
    <div class="p-8 max-w-6xl mx-auto space-y-8">
      
      <!-- Header -->
      <div class="flex justify-between items-center">
        <div>
           <h1 class="text-3xl font-bold text-gray-800">Settings</h1>
           <p class="text-gray-500 mt-1">Manage application preferences and screenshot automation</p>
        </div>
      </div>

      <!-- Screenshot Configuration -->
      <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
         <div class="px-6 py-4 border-b border-gray-100 bg-gray-50 flex justify-between items-center">
            <h2 class="text-lg font-semibold text-gray-800 flex items-center space-x-2">
                <Camera class="w-5 h-5 text-primary" />
                <span>Intelligent Screenshots</span>
            </h2>
            <div class="flex items-center space-x-2">
                <span :class="isEnabled ? 'text-green-600 bg-green-50' : 'text-gray-500 bg-gray-100'" class="px-3 py-1 rounded-full text-xs font-medium border border-transparent">
                    {{ isEnabled ? 'Active' : 'Disabled' }}
                </span>
            </div>
         </div>
         
         <div class="p-6 space-y-8">
             <!-- Toggle & Interval -->
             <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                 <div class="space-y-4">
                     <label class="flex items-center justify-between p-4 border rounded-lg cursor-pointer transition-all hover:border-primary-300" :class="isEnabled ? 'border-primary bg-primary-50' : 'border-gray-200'">
                         <div>
                             <span class="font-medium text-gray-800 block">Enable Auto-Screenshots</span>
                             <span class="text-xs text-gray-500">Automatically capture screen activity</span>
                         </div>
                         <div class="relative inline-flex items-center cursor-pointer">
                             <input type="checkbox" v-model="isEnabled" class="sr-only peer">
                             <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary"></div>
                         </div>
                     </label>

                     <div>
                         <label class="block text-sm font-medium text-gray-700 mb-2">Capture Frequency</label>
                         <select v-model="intervalMinutes" :disabled="!isEnabled" class="w-full border-gray-300 rounded-lg shadow-sm focus:border-primary focus:ring focus:ring-primary-200 focus:ring-opacity-50 disabled:bg-gray-100 disabled:text-gray-400">
                             <option v-for="opt in intervals" :key="opt.value" :value="opt.value">
                                 {{ opt.label }}
                             </option>
                         </select>
                     </div>
                 </div>

                 <!-- Description / Info -->
                 <div class="bg-blue-50 p-4 rounded-lg text-sm text-blue-700 space-y-2">
                     <p class="font-semibold flex items-center"><Monitor class="w-4 h-4 mr-2"/> How it works</p>
                     <p>The system intelligently categorizes your activity based on the active window title.</p>
                     <ul class="list-disc list-inside space-y-1 ml-1 opacity-80">
                         <li><strong>Work</strong>: Coding, Terminals, Office Apps</li>
                         <li><strong>Meeting</strong>: Zoom, Teams, Slack</li>
                         <li><strong>Break</strong>: Youtube, Netflix, Games</li>
                     </ul>
                 </div>
             </div>

             <!-- Manual Action -->
             <div class="border-t pt-6 flex items-center justify-between">
                 <div>
                    <button @click="handleTakeNow" :disabled="isTaking" class="btn-primary flex items-center space-x-2 px-4 py-2 bg-gray-800 hover:bg-gray-900 text-white rounded-md transition-all shadow-sm">
                        <Camera v-if="!isTaking" class="w-4 h-4" />
                        <span v-else class="animate-spin w-4 h-4 border-2 border-white border-t-transparent rounded-full"></span>
                        <span>{{ isTaking ? 'Capturing...' : 'Take Screenshot Now' }}</span>
                    </button>
                    <p v-if="successMsg" class="text-green-600 text-xs mt-2 flex items-center"><Check class="w-3 h-3 mr-1" /> {{ successMsg }}</p>
                    <p v-if="errorMsg" class="text-red-500 text-xs mt-2 flex items-center"><AlertCircle class="w-3 h-3 mr-1" /> {{ errorMsg }}</p>
                 </div>
             </div>
         </div>
      </div>

      <!-- History -->
      <div v-if="history.length > 0" class="space-y-4">
          <div class="flex justify-between items-center">
              <h3 class="text-xl font-bold text-gray-800 flex items-center"><History class="w-5 h-5 mr-2 text-gray-500" /> Recent Captures</h3>
              <button @click="clearHistory" class="text-red-500 hover:text-red-700 text-sm flex items-center">
                  <Trash2 class="w-4 h-4 mr-1" /> Clear History
              </button>
          </div>
          
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
              <div v-for="(log, idx) in history" :key="idx" class="bg-white rounded-lg shadow-sm border border-gray-100 overflow-hidden hover:shadow-md transition-shadow group">
                  <div class="h-40 overflow-hidden bg-gray-100 relative">
                     <img :src="convertFileSrc(log.path)" class="w-full h-full object-cover object-top transition-transform group-hover:scale-105" loading="lazy" />
                     <div class="absolute top-2 right-2 px-2 py-1 bg-black/70 text-white text-[10px] rounded backdrop-blur-sm">
                         {{ log.category }}
                     </div>
                  </div>
                  <div class="p-3">
                      <p class="text-xs text-gray-500 mb-1">{{ log.path.split('\\').pop()?.split('/').pop() }}</p>
                      <p class="text-xs font-medium text-gray-700 flex items-center">
                          <Clock class="w-3 h-3 mr-1" />
                          {{ log.timestamp }}
                      </p>
                  </div>
              </div>
          </div>
      </div>

    </div>
  </MainLayout>
</template>

<style scoped>
.btn-primary {
    @apply focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900;
}
</style>
