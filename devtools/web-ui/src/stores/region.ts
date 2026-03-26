// devtools/web-ui/src/stores/region.ts

import { defineStore } from 'pinia';
import * as wasm from '../wasm/map_editor.js'; // Ajustá según tu build de wasm-bindgen

interface Region {
  id: string;
  name: string;
  color: string;
  stateIds: string[]; // Lista de estados que pertenecen a esta región
}

export const useRegionStore = defineStore('regions', {
  state: () => ({
    regions: [] as Region[],
    activeRegionId: null as string | null,
  }),

  actions: {
    createRegion(name: string, color: string) {
      const id = `reg_${Date.now()}`;
      this.regions.push({ id, name, color, stateIds: [] });
      this.setActiveRegion(id);
    },

    setActiveRegion(id: string | null) {
      this.activeRegionId = id;
      // Llamada crucial al Bridge de Rust que definimos antes
      wasm.set_active_region(id);
    },

    // Esta función la llamará el Bridge (outbound) cuando Bevy detecte el click
    addStateToRegion(stateId: string, regionId: string) {
      const region = this.regions.find(r => r.id === regionId);
      if (region && !region.stateIds.includes(stateId)) {
        // Quitamos el estado de cualquier otra región (un estado solo puede estar en una)
        this.regions.forEach(r => {
          r.stateIds = r.stateIds.filter(id => id !== stateId);
        });
        region.stateIds.push(stateId);
      }
    }
  }
});