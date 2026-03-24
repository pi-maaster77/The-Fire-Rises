// frontend/web-ui/src/stores/game.ts

import { defineStore } from 'pinia';
// Este import vendrá del build de wasm-pack
import { update_bevy_counter } from '../../wasm-map/map_engine';

export const useGameStore = defineStore('game', {
  state: () => ({
    counter: 0,
  }),
  actions: {
    // Cuando el usuario cambia el contador en la UI de Vue
    updateFromUI(val: number) {
      this.counter = val;
      update_bevy_counter(val); // Avisamos a Bevy
    },
    // Cuando Bevy nos avisa de un cambio interno
    setCounterFromBevy(val: number) {
      this.counter = val;
    },
		hackBevy(){
			send_to_bevy(900)
		}
  }
});