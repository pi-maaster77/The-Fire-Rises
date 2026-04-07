<!-- devtools/src/components/domain/GameView.vue -->

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
