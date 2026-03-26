// devtools/web-ui/src/stores/states.ts

import { defineStore } from 'pinia';
import { select_province_by_id } from '../wasm/map_editor.js';

type State = {
	id: string;
	name: string;
	provinces: string[];
}

export const useStatesStore = defineStore('states', {
	state: () => ({
		selectedProvince: null as any,
		selectedState: null as any,
		states: [] as State[],
	}),
	actions: {
		// Esta es la función que llamarás desde tu componente de Vue
		selectFromUI(id: string) {
			// 1. Le avisamos a Rust para que el mapa se pinte dorado
			select_province_by_id(id);
			
			// 2. Actualizamos el estado local de Vue
			this.selectedProvince = { id };
		},
		addState(id:string, name:string){
			this.states.push({id, name, provinces:[]});
		}
	}
});