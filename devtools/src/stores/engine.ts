/*
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/stores/engine.ts  *
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
