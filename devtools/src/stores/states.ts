// devtools/src/stores/states.ts

import { defineStore } from 'pinia'
import type { State } from '@/types/Map'

export const useStateStore = defineStore('map', {
  state: () => ({
    states: [] as State[],
    selectedState: null as State | null,
    isBrushing: false,
  }),
  actions: {
    add(data: State) {
      if (this.states.some((state) => state.id === data.id)) {
        return
      }
      const regex = /^[A-Z0-9]{3}$/

      if (!regex.test(data.id)) {
        console.error(
          'ID del estado inválido. Debe ser de 3 caracteres alfanuméricos en mayúscula.',
        )
        return
      }
      this.states.push(data)
    },
    remove(id: string) {
      this.states = this.states.filter((state) => state.id !== id)
    },
    update(id: string, data: State) {
      const regex = /^[A-Z0-9]{3}$/
      if (!regex.test(data.id)) {
        console.error(
          'ID del estado inválido. Debe ser de 3 caracteres alfanuméricos en mayúscula.',
        )
        return
      }
      const index = this.states.findIndex((state) => state.id === id)
      if (index === -1) return
      this.states[index] = data
    },
    get(id: string) {
      return this.states.find((state) => state.id === id)
    },
    clear() {
      this.states = []
    },
    load(data: State[]) {
      this.states = data
    },
    selectState(id: string | null) {
      this.selectedState = this.states.find((state) => state.id === id) || null
    },
    assignProvinceToState(stateId: string, provinceIndex: number) {
      const state = this.states.find((s) => s.id === stateId)
      if (state && !state.provinces.includes(provinceIndex)) {
        state.provinces.push(provinceIndex)
      }
      // Opcional: Quitar la provincia de otros estados si es necesario
      this.states.forEach((s) => {
        if (s.id !== stateId) {
          s.provinces = s.provinces.filter((index) => index !== provinceIndex)
        }
      })
    },
  },
})
