// frontend/web-ui/src/stores/game.ts

import { defineStore } from 'pinia';
// Importamos la función que expusimos en Rust
import { send_increment_to_rust } from '../wasm/map_engine';

export const useGameStore = defineStore('game', {
  state: () => ({
    counter: 0,
    countrySelected: 'Argentina',
  }),
  actions: {
    increment(amount: number) {
      this.counter += amount;
      
      // Notificamos al motor de Rust
      // Esto asegura que Bevy y Vue estén sincronizados
      send_increment_to_rust(amount);
    }
  }
});