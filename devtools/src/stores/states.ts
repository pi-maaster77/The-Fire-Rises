/*
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/stores/states.ts  *
 * Project: The Fire Rises                                                     *
 * Created Date: Mo Jun 2026                                                   *
 * Author: pi-maaster77 (pimaaster1337@gmail.com)                              *
 * -----                                                                       *
 * Last Modified: Mon Jun 08 2026                                              *
 * Modified By: pi-maaster77                                                   *
 * -----                                                                       *
 * Copyright (c) projectCreationYear-2026 The Fire Rises                       *
 * -----                                                                       *
 * License: GNU Affero General Public License v3.0 (AGPL-3.0)                  *
 * This program is free software: you can redistribute it and/or modify        *
 * it under the terms of the GNU Affero General Public License as              *
 * published by the Free Software Foundation, either version 3 of the          *
 * License, or (at your option) any later version.                             *
 * -----                                                                       *
 * HISTORY:                                                                    *
 * Date      	By	Comments                                                     *
 * ----------	---	---------------------------------------------------------    *
 */

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
