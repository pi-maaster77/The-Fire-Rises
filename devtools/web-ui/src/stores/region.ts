// devtools/web-ui/src/stores/region.ts
import { defineStore } from 'pinia';
import { useStatesStore } from './states';

export interface Region {
  id: string;
  name: string;
  color: string;
  stateIds: string[];
}

export const useRegionStore = defineStore('regions', {
  state: () => ({
    regions: [] as Region[],
    activeRegionId: null as string | null,
  }),

  getters: {
    activeRegion: (state) => state.regions.find(r => r.id === state.activeRegionId),
    
    // Estados que aún no tienen región asignada
    unassignedStates: (state) => {
      const statesStore = useStatesStore();
      const assigned = state.regions.flatMap(r => r.stateIds);
      return statesStore.allStateIds.filter(id => !assigned.includes(id));
    }
  },

  actions: {
    createRegion(id: string, name: string, color: string) {
      this.regions.push({ id, name, color, stateIds: [] });
      this.activeRegionId = id;
    },

    // 100% UI: No depende de clicks en el mapa
    assignStateToRegion(stateId: string) {
      // Un estado solo pertenece a una región
      this.regions.forEach(r => {
        r.stateIds = r.stateIds.filter(id => id !== stateId);
      });

      const region = this.regions.find(r => r.id === this.activeRegion?.id);
      if (region && !region.stateIds.includes(stateId)) {
        region.stateIds.push(stateId);
      }

      const statesStore = useStatesStore();
      statesStore.setStateRegion(stateId, this.activeRegion?.id ?? null);
      
      this.syncWithRust();
    },

    removeStateFromRegion(stateId: string) {
      if (this.activeRegion) {
        this.activeRegion.stateIds = this.activeRegion.stateIds.filter(id => id !== stateId);
        this.syncWithRust();
      }
    },

    syncWithRust() {
      // Aquí podrías llamar a una función de Rust que pinte todas las regiones de un saque
      // update_bevy_region_colors(JSON.stringify(this.regions));
    }
  }
});