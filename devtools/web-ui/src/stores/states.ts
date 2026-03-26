// devtools/web-ui/src/stores/states.ts

import { defineStore } from 'pinia';
import { select_province_by_id, assign_province_to_state } from '../wasm/map_editor.js';

type State = {
	id: string;
	name: string;
	provinces: string[];
}

export const useStatesStore = defineStore('states', {
  state: () => ({
    selectedProvinceId: null as string | null,
    selectedStateId: null as string | null,
    states: [] as State[],
  }),
  actions: {
    assignSelectedToState(stateId: string) {
      if (!this.selectedProvinceId) return;

      // 1. Actualizar lógica local en Vue
      const state = this.states.find(s => s.id === stateId);
      if (state && !state.provinces.includes(this.selectedProvinceId)) {
        state.provinces.push(this.selectedProvinceId);
      }

      // 2. Avisar a Rust para que repinte el mapa con el color del estado
      // Pasamos la provincia y el ID del estado
      assign_province_to_state(this.selectedProvinceId, stateId);
    },
				// Esta es la función que llamarás desde tu componente de Vue
		selectFromUI(id: string) {
			// 1. Le avisamos a Rust para que el mapa se pinte dorado
			select_province_by_id(id);
			
			// 2. Actualizamos el estado local de Vue
			this.selectedProvinceId = id;
		},
		addState(id:string, name:string){
			this.states.push({id, name, provinces:[]});
		}
  }
});