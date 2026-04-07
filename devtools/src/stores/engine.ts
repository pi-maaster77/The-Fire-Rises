// devtools/src/stores/engine.ts

import { defineStore } from 'pinia'
import { markRaw } from 'vue'
import type { MapEngine } from '@/engine/MapEngine'

export const useEngineStore = defineStore('engine', {
  state: () => ({
    // Usamos markRaw para que Vue no intente hacer reactivo todo el objeto de Pixi
    // (Pixi tiene miles de propiedades internas y la reactividad de Vue lo mataría)
    instance: null as MapEngine | null,
    isReady: false,
  }),
  actions: {
    setEngine(engine: MapEngine) {
      this.instance = markRaw(engine)
      this.isReady = true
    },
  },
  getters: {
    map: (state) => state.instance,
  },
})
