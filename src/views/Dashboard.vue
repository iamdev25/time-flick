<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MainLayout from '../components/MainLayout.vue';
import { Play, Pause, Square, Clock } from 'lucide-vue-next';

interface TimeInfo {
  formatted_time: string;
  seconds: number;
  is_running: boolean;
}

interface TimeLog {
  id: number;
  startTime: string; // ISO string
  duration: string;
  seconds: number;
  note: string;
}

const currentTime = ref('00:00:00');
const isRunning = ref(false);
const timerInterval = ref<number | null>(null);

const timeLogs = ref<TimeLog[]>([]);

// Load logs from local storage
const loadLogs = () => {
  const saved = localStorage.getItem('timeLogs');
  if (saved) {
    timeLogs.value = JSON.parse(saved);
  }
};

const saveLogs = () => {
  localStorage.setItem('timeLogs', JSON.stringify(timeLogs.value));
};

const updateTimer = async () => {
  try {
    const info = await invoke<TimeInfo>('get_timer_time');
    currentTime.value = info.formatted_time;
    isRunning.value = info.is_running;
  } catch (e) {
    console.error(e);
  }
};

const startTimer = async () => {
  try {
    const info = await invoke<TimeInfo>('start_timer');
    currentTime.value = info.formatted_time;
    isRunning.value = info.is_running;
    startPolling();
  } catch (e) {
    console.error(e);
  }
};

const pauseTimer = async () => {
  try {
    const info = await invoke<TimeInfo>('pause_timer');
    currentTime.value = info.formatted_time;
    isRunning.value = info.is_running;
    stopPolling();
  } catch (e) {
    console.error(e);
  }
};

const stopTimer = async () => {
  try {
    const info = await invoke<TimeInfo>('stop_timer');
    // Save log
    if (info.seconds > 0) {
       const newLog: TimeLog = {
         id: Date.now(),
         startTime: new Date().toISOString(),
         duration: info.formatted_time,
         seconds: info.seconds,
         note: 'Work Session' // Placeholder
       };
       timeLogs.value.unshift(newLog);
       saveLogs();
    }
    
    currentTime.value = "00:00:00";
    isRunning.value = false;
    stopPolling();
  } catch (e) {
    console.error(e);
  }
};

const startPolling = () => {
  if (timerInterval.value) return;
  timerInterval.value = window.setInterval(updateTimer, 1000);
};

const stopPolling = () => {
  if (timerInterval.value) {
    clearInterval(timerInterval.value);
    timerInterval.value = null;
  }
};

const totalTimeToday = computed(() => {
  // Simple check for "today" based on ISO date string
  const today = new Date().toISOString().split('T')[0];
  const totalSeconds = timeLogs.value
    .filter(log => log.startTime.startsWith(today))
    .reduce((acc, log) => acc + log.seconds, 0);
  
  const h = Math.floor(totalSeconds / 3600);
  const m = Math.floor((totalSeconds % 3600) / 60);
  const s = totalSeconds % 60;
  return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
});

onMounted(() => {
  loadLogs();
  updateTimer().then(() => {
    if (isRunning.value) {
      startPolling();
    }
  });
});

onUnmounted(() => {
  stopPolling();
});

</script>

<template>
  <MainLayout :is-running="isRunning" :current-time="currentTime">
    <div class="max-w-4xl mx-auto space-y-8">
      <!-- Timer Control Card -->
      <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-8 flex flex-col items-center justify-center space-y-6">
        <div class="text-center">
          <p class="text-sm text-gray-400 font-medium uppercase tracking-wider mb-2">Current Session</p>
          <div class="text-7xl font-bold text-gray-800 font-mono tracking-tighter">
            {{ currentTime }}
          </div>
        </div>
        
        <div class="flex items-center space-x-4">
           <button 
             v-if="!isRunning"
             @click="startTimer"
             class="flex items-center space-x-2 bg-primary hover:bg-primary-hover text-white px-8 py-3 rounded-full font-bold shadow-lg shadow-primary/30 transition-all transform hover:scale-105 active:scale-95 text-lg"
           >
             <Play class="w-6 h-6" />
             <span>{{ currentTime !== '00:00:00' ? 'Resume' : 'Start' }}</span>
           </button>
           
           <button 
             v-else
             @click="pauseTimer"
             class="flex items-center space-x-2 bg-amber-500 hover:bg-amber-600 text-white px-8 py-3 rounded-full font-bold shadow-lg shadow-amber-500/30 transition-all transform hover:scale-105 active:scale-95 text-lg"
           >
             <Pause class="w-6 h-6" />
             <span>Pause</span>
           </button>
           
           <button 
             @click="stopTimer"
             class="flex items-center space-x-2 bg-gray-200 hover:bg-gray-300 text-gray-700 px-6 py-3 rounded-full font-bold transition-all"
             :disabled="currentTime === '00:00:00'"
             :class="{ 'opacity-50 cursor-not-allowed': currentTime === '00:00:00' }"
           >
             <Square class="w-6 h-6 fill-current" />
             <span>Stop</span>
           </button>
        </div>
      </div>
      
      <!-- Stats Row -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-100 flex items-center space-x-4">
           <div class="p-3 bg-blue-50 text-blue-600 rounded-full">
             <Clock class="w-6 h-6" />
           </div>
           <div>
             <p class="text-sm text-gray-500 font-medium">Today's Total</p>
             <p class="text-2xl font-bold text-gray-800">{{ totalTimeToday }}</p>
           </div>
        </div>
        <!-- Placeholders for other stats -->
        <div class="bg-white p-6 rounded-lg shadow-sm border border-gray-100 flex items-center space-x-4">
           <div class="p-3 bg-purple-50 text-purple-600 rounded-full">
             <Clock class="w-6 h-6" />
           </div>
           <div>
             <p class="text-sm text-gray-500 font-medium">This Week</p>
             <p class="text-2xl font-bold text-gray-800">--:--:--</p>
           </div>
        </div>
      </div>

      <!-- Recent Activity -->
      <div class="bg-white rounded-lg shadow-sm border border-gray-100 overflow-hidden">
        <div class="px-6 py-4 border-b border-gray-100 flex justify-between items-center">
           <h3 class="text-lg font-semibold text-gray-800">Recent Activity</h3>
           <button v-if="timeLogs.length > 0" @click="timeLogs = []; saveLogs()" class="text-xs text-red-500 hover:text-red-700">Clear History</button>
        </div>
        <div v-if="timeLogs.length === 0" class="p-8 text-center text-gray-400">
           No activity recorded yet.
        </div>
        <ul v-else class="divide-y divide-gray-100">
           <li v-for="log in timeLogs" :key="log.id" class="px-6 py-4 flex justify-between items-center hover:bg-gray-50 transition">
             <div>
               <p class="text-sm font-medium text-gray-800">{{ log.note }}</p>
               <p class="text-xs text-gray-500">{{ new Date(log.startTime).toLocaleString() }}</p>
             </div>
             <p class="text-sm font-bold font-mono text-gray-700">{{ log.duration }}</p>
           </li>
        </ul>
      </div>
    </div>
  </MainLayout>
</template>
