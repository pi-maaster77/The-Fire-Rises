// frontend/web-ui/src/main.ts

import { createApp } from 'vue';
import { createPinia } from 'pinia'; // 1. Importa Pinia
import './style.css';
import App from './App.vue';
import { useGameStore } from './stores/game';

const app = createApp(App);
const pinia = createPinia(); // 2. Crea la instancia

app.use(pinia); // 3. Instálalo en la app
app.mount('#app');

// 4. Ahora sí definimos el Bridge
(window as any).__BEVY_BRIDGE__ = {
    on_bevy_counter_update: (val: number) => {
        // Ahora Pinia ya está activo y esto no fallará
        const store = useGameStore();
        if (store.counter !== val) {
            store.setCounterFromBevy(val);
        }
    }
};