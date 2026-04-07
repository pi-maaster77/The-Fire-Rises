<!-- devtools/src/App.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { MapEngine } from './engine/MapEngine'
import mapDataJson from './data/map_test.json' // Ajusta la ruta de tu JSON
import type { MapData } from '@/types/Map'
import FileInput from './components/ui/input/FileInput.vue'
import ToolBox from './components/layout/ToolBox.vue'

const fps = ref(0)
const canvasContainer = ref<HTMLElement | null>(null)
const engine = ref<MapEngine | null>(null)

const updateFPS = () => {
  if (engine.value?.app.ticker) {
    // Math.round para no tener decimales molestos
    fps.value = Math.round(engine.value.app.ticker.FPS)
  }
}

onMounted(async () => {
  if (canvasContainer.value) {
    const engineInstance = new MapEngine(canvasContainer.value)
    engine.value = engineInstance

    await engineInstance.initPromise
    engineInstance.app.ticker.add(updateFPS)
    // Esperamos un frame para asegurar que Pixi se inicializó
    setTimeout(() => {
      engine.value?.renderMap(mapDataJson as unknown as MapData)
    }, 100)
  }
})

function onFileUpload(event: Event) {
  const input = event.target as HTMLInputElement

  if (!input.files || input.files.length === 0) return

  const file = input.files[0]

  const reader = new FileReader()

  reader.onload = (e) => {
    try {
      const text = e.target?.result as string
      const json = JSON.parse(text) as MapData

      console.log('Mapa cargado:', json)

      engine.value?.renderMap(json)
    } catch (err) {
      console.error('Error al parsear JSON:', err)
      alert('Archivo inválido')
    }
  }

  if (file) {
    reader.readAsText(file)
  }
}
</script>

<template>
  <main>
    <div ref="canvasContainer" class="map-viewport"></div>

    <div class="ui-overlay">
      <div class="stats"></div>
      <FileInput @change="onFileUpload"> ⬆️ </FileInput>
    </div>

    <ToolBox />

    <div class="fps-counter">
      <p>FPS: {{ fps }}</p>
    </div>
  </main>
</template>

<style scoped>
.map-viewport {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
  overflow: hidden;
}

.fps-counter {
  position: absolute;
  z-index: 1000;
  top: 10px;
  right: 10px;
  p {
    color: white;
    font-family: 'Courier New', Courier, monospace;
  }
}

.ui-overlay {
  position: relative;
  z-index: 10;
  pointer-events: none; /* Los clicks pasan de largo por el fondo del div */
  color: white;
  padding: 20px;
  font-family: 'Courier New', Courier, monospace;
}

/* Hacemos que los controles SÍ reaccionen al mouse */
.ui-overlay * {
  pointer-events: auto;
}
</style>
