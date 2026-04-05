// devtools/web-ui/src/stores/map.ts

import { defineStore } from 'pinia';
import { select_province_by_id, trigger_export_map } from '../wasm/map_editor.js';

export const useMapStore = defineStore('map', {
  state: () => ({
    selectedProvince: null as any,
    selectedState: null as any,
    provinces: [] as string[],
    exportedMap: null as Record<string, any> | null,
  }),
  actions: {
    selectFromUI(id: string) {
      select_province_by_id(id);
      this.selectedProvince = { id };
    },

    exportMap() {
      trigger_export_map();
    },

    setExportedMap(payload: Record<string, any>) {
      this.exportedMap = payload;
    },

    downloadExportedMap() {
      if (!this.exportedMap) return;
      const content = JSON.stringify(this.exportedMap, null, 2);
      const blob = new Blob([content], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'map-export.json';
      a.click();
      URL.revokeObjectURL(url);
    }
  }
});