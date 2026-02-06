import { ref, watch, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface ScreenshotLog {
    path: string;
    category: string;
    timestamp: string;
}

const isEnabled = ref(false);
const intervalMinutes = ref(10);
const history = ref<ScreenshotLog[]>([]);
const timerId = ref<number | null>(null);

export function useScreenshotSystem() {

    const loadSettings = () => {
        const settings = localStorage.getItem('screenshotSettings');
        if (settings) {
            const parsed = JSON.parse(settings);
            isEnabled.value = parsed.isEnabled;
            intervalMinutes.value = parsed.intervalMinutes;
        }
        const savedHistory = localStorage.getItem('screenshotHistory');
        if (savedHistory) {
            history.value = JSON.parse(savedHistory);
        }
    };

    const saveSettings = () => {
        localStorage.setItem('screenshotSettings', JSON.stringify({
            isEnabled: isEnabled.value,
            intervalMinutes: intervalMinutes.value
        }));
    };

    const saveHistory = () => {
        // Keep last 50
        if (history.value.length > 50) {
            history.value = history.value.slice(0, 50);
        }
        localStorage.setItem('screenshotHistory', JSON.stringify(history.value));
    };

    const takeScreenshot = async () => {
        try {
            const result = await invoke<ScreenshotLog>('capture_screenshot');
            history.value.unshift(result);
            saveHistory();
            return result;
        } catch (e) {
            console.error('Failed to take screenshot:', e);
            throw e;
        }
    };

    const startTimer = () => {
        stopTimer();
        if (!isEnabled.value) return;

        // Convert minutes to ms
        const ms = intervalMinutes.value * 60 * 1000;
        console.log(`Starting screenshot timer: every ${intervalMinutes.value} mins (${ms}ms)`);

        timerId.value = window.setInterval(async () => {
            await takeScreenshot();
        }, ms);
    };

    const stopTimer = () => {
        if (timerId.value) {
            clearInterval(timerId.value);
            timerId.value = null;
        }
    };

    // Watchers to restart timer if settings change
    watch([isEnabled, intervalMinutes], () => {
        saveSettings();
        if (isEnabled.value) {
            startTimer();
        } else {
            stopTimer();
        }
    });

    // Initial load
    onMounted(() => {
        loadSettings();
        if (isEnabled.value && !timerId.value) {
            startTimer();
        }
    });

    // Note: we don't necessarily want to query stopTimer on unmount if this is a global system,
    // but since this is a composable used in a component, and the app is likely valid as long as window is open...
    // Actually, if we use this in Settings.vue, unmounting Settings shouldn't stop the timer if we want it global.
    // Ideally, this state should be in a global store or App.vue.
    // But given standard Vue composable patterns, if we want it global, we lift the state outside the function (which I did).
    // However, `onMounted` triggers every time the composable is used.
    // We should prevent `startTimer` if already running?
    // Or better, just put the logic in `App.vue` once, and `Settings.vue` only modifies state.

    return {
        isEnabled,
        intervalMinutes,
        history,
        takeScreenshot,
        loadSettings
    };
}
