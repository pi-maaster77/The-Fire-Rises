// frontend/web-ui/src/stores/map.ts

import { defineStore } from 'pinia';

export const useMapStore = defineStore('map', {
  state: () => ({
    selectedProvince: null as any,
    selectedState: null as any,
  }),
});