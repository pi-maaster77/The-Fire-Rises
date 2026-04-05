<!-- devtools/src/components/domain/GameView.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { MapEngine } from '../../engine/MapEngine'

const mapContainer = ref<HTMLElement | null>(null)
let engine: MapEngine | null = null

onMounted(() => {
  if (mapContainer.value) {
    engine = new MapEngine(mapContainer.value)
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
