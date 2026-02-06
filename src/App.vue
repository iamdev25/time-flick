<script setup lang="ts">
import { useScreenshotSystem } from './composables/useScreenshotSystem';

// Initialize global systems
useScreenshotSystem();
</script>

<template>
  <router-view />
</template>