<!--
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/components/domain/systems/BrushSystem.vue * 
 * Project: The Fire Rises                                                    * 
 * Created Date: Mo Jun 2026                                                  *
 * Author: pi-maaster77 (pimaaster1337@gmail.com)                             *
 * -----                                                                      *
 * Last Modified: Mon Jun 08 2026                                             * 
 * Modified By: pi-maaster77                                                  * 
 * -----                                                                      *
 * Copyright (c) projectCreationYear-2026 The Fire Rises                      *
 * -----                                                                      *
 * License: GNU Affero General Public License v3.0 (AGPL-3.0)                 *
 * This program is free software: you can redistribute it and/or modify       *
 * it under the terms of the GNU Affero General Public License as             *
 * published by the Free Software Foundation, either version 3 of the         *
 * License, or (at your option) any later version.                            *
 * -----                                                                      *
 * HISTORY:                                                                   *
 * Date      	By	Comments                                                    *
 * ----------	---	---------------------------------------------------------   *
-->

<script setup lang="ts">
import { useStateStore } from '@/stores/states'
import { useEngineStore } from '@/stores/engine' // <--- Nueva store
import { computed } from 'vue'

const stateStore = useStateStore()
const engineStore = useEngineStore()

const selectedStateId = computed({
  get: () => stateStore.selectedState?.id,
  set: (value: string) => stateStore.selectState(value),
})

function toggleBrush() {
  if (!stateStore.selectedState) return

  stateStore.isBrushing = !stateStore.isBrushing

  // Acceso directo y limpio
  const engine = engineStore.instance
  if (!engine) return

  engine.setCameraControl(!stateStore.isBrushing)

  if (stateStore.isBrushing) {
    engine.onBrushStroke = (index) => {
      const color = parseInt(stateStore.selectedState!.color.replace('#', '0x'))
      engine.updateProvinceVisual(index, color)
      stateStore.assignProvinceToState(stateStore.selectedState!.id, index)
    }
  } else {
    engine.onBrushStroke = undefined
  }
}
</script>

<template>
  <div class="brush-tool">
    <select v-model="selectedStateId">
      <option disabled value="">Seleccionar Estado</option>
      <option v-for="state in stateStore.states" :key="state.id" :value="state.id">
        {{ state.name }}
      </option>
    </select>

    <button @click="toggleBrush" :class="{ 'btn-active': stateStore.isBrushing }">
      {{ stateStore.isBrushing ? '🖌️ Pincel Activo' : '🖐️ Modo Cámara' }}
    </button>
  </div>
</template>

<style scoped>
.brush-tool {
  background: rgba(0, 0, 0, 0.8);
  padding: 15px;
  border-radius: 8px;
  pointer-events: auto;
}
.btn-active {
  background-color: #ccff00;
  color: black;
  font-weight: bold;
}
</style>
