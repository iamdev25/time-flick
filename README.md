# Time Flick

A modern, desktop time tracking application built with Tauri, Vue 3, and Rust.

## Tech Stack

- **Frontend**: Vue 3 (Composition API), TypeScript, Tailwind CSS
- **Backend**: Tauri (Rust)
- **State**: In-memory Rust state (Timer), LocalStorage (Logs)

## Features

- **Timer**: Start, Pause, Resume, Stop. Logic runs in Rust backend.
- **Dashboard**: Real-time updates, Session history.
- **Persistence**: Logs are saved to local storage. Timer state persists while app is running (even if minimized).

## Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Run in development mode:
   ```bash
   npm run tauri dev
   ```

3. Build for production:
   ```bash
   npm run tauri build
   ```

## Structure

- `src-tauri/src/lib.rs`: Rust backend logic (Timer state, Commands).
- `src/views/Dashboard.vue`: Main UI, interacts with Rust backend.
- `src/components/`: Reusable UI components (Sidebar, TopBar).
