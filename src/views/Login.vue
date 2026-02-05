<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();

const handleLogin = () => {
  // Mock login logic
  localStorage.setItem('isAuthenticated', 'true');
  router.push('/dashboard');
};
</script>

<template>
  <div class="flex items-center justify-center h-screen bg-gray-100">
    <div class="bg-white p-8 rounded-lg shadow-lg w-96">
      <h1 class="text-2xl font-bold mb-6 text-center text-gray-800">Time Flick</h1>
      <p class="mb-6 text-gray-500 text-center">Sign in to start tracking</p>
      
      <button 
        @click="handleLogin"
        class="w-full bg-primary hover:bg-primary-hover text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-200"
      >
        Sign In
      </button>
    </div>
  </div>
</template>
