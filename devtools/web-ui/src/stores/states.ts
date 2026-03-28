// devtools/web-ui/src/stores/states.ts
import { defineStore } from 'pinia';
import { select_province_by_id, assign_province_to_state } from '../wasm/map_editor.js';

export interface State {
  id: string;
  name: string;
  provinces: string[];
}

export const useStatesStore = defineStore('states', {
  state: () => ({
    selectedProvinceId: null as string | null,
    states: [] as State[],
  }),
  
  getters: {
    // Getter para saber a qué estado pertenece la provincia seleccionada
    selectedProvinceState: (state) => {
      if (!state.selectedProvinceId) return null;
      return state.states.find(s => s.provinces.includes(state.selectedProvinceId!));
    },
    // Lista de IDs de estados para los selectores de la UI
    allStateIds: (state) => state.states.map(s => s.id),
  },

  actions: {
    // Centralizamos el registro de estados que Bevy descubre al escanear
    registerState(id: string, name: string) {
      if (!this.states.find(s => s.id === id)) {
        this.states.push({ id, name, provinces: [] });
      }
    },

    // Selección desde el mapa (Bevy -> Vue)
    setProvinceSelection(provinceId: string) {
      this.selectedProvinceId = provinceId;
    },

    // Selección desde la UI (Vue -> Rust)
    selectFromUI(id: string) {
      this.selectedProvinceId = id;
      select_province_by_id(id);
    },

    assignProvinceToState(provinceId: string, stateId: string) {
      // 1. Limpieza: quitar provincia de cualquier estado previo
      this.states.forEach(s => {
        s.provinces = s.provinces.filter(p => p !== provinceId);
      });

      // 2. Asignar al nuevo
      const state = this.states.find(s => s.id === stateId);
      if (state) {
        state.provinces.push(provinceId);
        assign_province_to_state(provinceId, stateId);
      }
    }
  }
});