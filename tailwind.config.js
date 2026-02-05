/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
           DEFAULT: '#10b981', // Emerald 500
           hover: '#059669',   // Emerald 600
        },
        dark: {
          bg: '#111827',       // Gray 900
          surface: '#1f2937',  // Gray 800
        }
      }
    },
  },
  plugins: [],
}
