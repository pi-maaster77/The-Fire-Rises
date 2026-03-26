// devtools/web-ui/src/stores/map.ts

import { defineStore } from 'pinia';
import { select_province_by_id } from '../wasm/map_editor.js';


export const useMapStore = defineStore('map', {
  state: () => ({
    selectedProvince: null as any,
    selectedState: null as any,
    provinces: [] as string[],
  }),
	actions: {
    // Esta es la función que llamarás desde tu componente de Vue
    selectFromUI(id: string) {
      // 1. Le avisamos a Rust para que el mapa se pinte dorado
      select_province_by_id(id);
      
      // 2. Actualizamos el estado local de Vue
      this.selectedProvince = { id };
    }
  }
});