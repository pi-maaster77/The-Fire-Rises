<!-- devtools/src/App.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { MapEngine } from './engine/MapEngine'

const canvasContainer = ref<HTMLElement | null>(null)

onMounted(() => {
  if (canvasContainer.value) {
    new MapEngine(canvasContainer.value)
  }
})
</script>

<template>
  <main>
    <div ref="canvasContainer" class="map-viewport"></div>

    <div class="ui-overlay">
      <h1>The Fire Rises</h1>
      <p>Año: 1985</p>
    </div>
  </main>
</template>

<style scoped>
.map-viewport {
  width: 100vw;
  height: 100vh;
  position: absolute;
  top: 0;
  left: 0;
  overflow: hidden;
}

.ui-overlay {
  position: relative;
  z-index: 10;
  pointer-events: none; /* Para que los clicks pasen al mapa */
  color: white;
  padding: 20px;
  font-family: 'Courier New', Courier, monospace;
}
</style>
