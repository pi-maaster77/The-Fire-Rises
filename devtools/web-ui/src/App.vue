<!-- devtools/web-ui/src/App.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import init, { load_map_image } from './wasm/map_editor.js';
import wasmUrl from './wasm/map_editor_bg.wasm?url';
import { setupBridge } from './wasm/bridge.js';
import { useMapStore } from './stores/map.js';

const mapStore = useMapStore();
const canvasRef = ref<HTMLCanvasElement | null>(null);
const isLoading = ref(false);

onMounted(async () => {
  // 1. Forzamos el tamaño del buffer ANTES de que Bevy lo toque
  if (canvasRef.value) {
    const canvas = canvasRef.value;
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
  }

	setupBridge();

  try {
		await new Promise(r => requestAnimationFrame(r));
    await init({ module_or_path: wasmUrl });
  } catch (error: any) {
    if (!error.toString().includes("exceptions for control flow")) {
      console.error("Error al iniciar el motor:", error);
    }
  }
});

const handleFileUpload = async (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;

  isLoading.value = true;

  const img = new Image();
  img.onload = () => {
    const canvas = document.createElement('canvas');
    canvas.width = img.width;
    canvas.height = img.height;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    ctx.drawImage(img, 0, 0);
    const imageData = ctx.getImageData(0, 0, img.width, img.height);
    
    // LA SOLUCIÓN: Convertir la vista del buffer
    // Usamos .buffer para acceder a la memoria cruda y crear el tipo que Rust espera
    const bytes = new Uint8Array(imageData.data.buffer); 

    // Ahora sí, llamamos a la función de WASM
    load_map_image(bytes, img.width, img.height);
    
    isLoading.value = false;
    URL.revokeObjectURL(img.src); // Limpieza de memoria
  };
  img.src = URL.createObjectURL(file);
};

const acceptMap = () => {
  alert('Mapa aceptado!'); // Placeholder
  // Aquí podrías enviar un evento a Bevy o manejar la lógica
};
</script>

<template>
  <div class="game-wrapper">
    <div class="ui-layer">
      <div class="panel">
        <h3>Map Editor</h3>
        <input type="file"
          id="map-upload"
          accept="image/png"
          @change="handleFileUpload"
        >
        <div v-if="isLoading" class="loading">
          <p>Cargando mapa...</p>
          <div class="spinner"></div>
        </div>
        <div v-else-if="mapStore.provinces.length > 0" class="map-ready">
          <p>Mapa cargado con {{ mapStore.provinces.length }} provincias</p>
          <button @click="acceptMap">Aceptar Mapa</button>
        </div>
        <div v-if="mapStore.selectedProvince">
          <h4>Provincia Seleccionada:</h4>
          <p>ID: {{ mapStore.selectedProvince.id }}</p>
          <p>Centro: {{ mapStore.selectedProvince.center }}</p>
        </div>
      </div>
    </div>
    <div class="canvas-container">
      <canvas id="canvas" ref="canvasRef"></canvas>
    </div>
  </div>

</template>

<style>
#canvas {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
	z-index: 1;
}

.game-wrapper {
  position: relative;
	display: flex;
	flex-direction: column
}

.ui-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 20vw;
	height: 100vh;
	z-index: 10;
	background-color: #bbb;
	padding: 10px;
}

.panel {
  background: white;
  padding: 10px;
  border-radius: 5px;
}

.loading {
  text-align: center;
  margin-top: 10px;
}

.spinner {
  border: 4px solid #f3f3f3;
  border-top: 4px solid #3498db;
  border-radius: 50%;
  width: 30px;
  height: 30px;
  animation: spin 2s linear infinite;
  margin: 10px auto;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.map-ready {
  margin-top: 10px;
}

button {
  padding: 5px 10px;
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 3px;
  cursor: pointer;
}

button:hover {
  background: #45a049;
}
</style>