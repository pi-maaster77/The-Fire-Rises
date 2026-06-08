<!--
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/components/domain/GameView.vue * 
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
import { onMounted, ref } from 'vue'
import { MapEngine } from '@/engine/MapEngine'
import { useEngineStore } from '@/stores/engine'

const canvasContainer = ref<HTMLElement | null>(null)
const engineStore = useEngineStore()

onMounted(async () => {
  if (canvasContainer.value) {
    const engine = new MapEngine(canvasContainer.value)
    await engine.initPromise

    // Guardamos la instancia en la store para que todos la usen
    engineStore.setEngine(engine)

    // Render inicial
    // engineStore.instance.renderMap(data)
  }
})
</script>

<template>
  <div ref="mapContainer" class="pixi-wrapper"></div>
</template>

<style scoped>
.pixi-wrapper {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  cursor: grab;
}
.pixi-wrapper:active {
  cursor: grabbing;
}
</style>
