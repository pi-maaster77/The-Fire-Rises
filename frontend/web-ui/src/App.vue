<!-- frontend/web-ui/src/App.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import init from './wasm/map_engine.js';
import wasmUrl from './wasm/map_engine_bg.wasm?url';
import { useGameStore } from './stores/game';
const gameStore = useGameStore();

const handleAdd = () => {
  gameStore.increment(1);
};

const canvasRef = ref<HTMLCanvasElement | null>(null);

onMounted(async () => {
  // 1. Forzamos el tamaño del buffer ANTES de que Bevy lo toque
  if (canvasRef.value) {
    const canvas = canvasRef.value;
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
		(window as any).updateVueCounter = (newValue: number) => {
    	gameStore.counter = newValue; // Sincronizamos el store
  	};
  }

  try {
    // 2. Esperamos un frame para asegurar que el DOM está "quieto"
    await new Promise(r => requestAnimationFrame(r));
    
    await init({ module_or_path: wasmUrl });
    console.log("Bevy cargado correctamente");
  } catch (error) {
    // Es normal ver el mensaje "Using exceptions for control flow" en la consola
    if (!error?.toString().includes("exceptions for control flow")) {
      console.error("Error real:", error);
    }
  }
});
</script>

<template>
  <div class="game-wrapper">
    <div class="ui-layer">
      <div class="panel">
        <h3>The Fire Rises</h3>
        <p>Contador: <strong>{{ gameStore.counter }}</strong></p>
        <button @click="handleAdd">Aumentar desde Vue</button>
      </div>
    </div>

    <div class="canvas-container">
      <canvas id="canvas" ref="canvasRef"></canvas>
    </div>
  </div>
</template>

<style scoped>
.game-wrapper {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: #1a1a1a;
}

.canvas-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1; /* El mapa va atrás */
}

#canvas {
  width: 100%;
  height: 100%;
  outline: none;
}

.ui-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 10; /* La UI va adelante */
  pointer-events: none; /* CLAVE: Los clicks pasan a través de la capa al canvas */
  padding: 20px;
}

.panel {
  pointer-events: auto; /* Reactivamos los clicks SOLO en el panel */
  width: 250px;
  background: rgba(20, 20, 25, 0.9);
  color: #eee;
  padding: 20px;
  border-radius: 12px;
  border: 1px solid #444;
  box-shadow: 0 8px 32px rgba(0,0,0,0.5);
}

button {
  width: 100%;
  padding: 10px;
  background: #42b883;
  border: none;
  border-radius: 6px;
  color: white;
  cursor: pointer;
  font-weight: bold;
}

button:hover {
  background: #33a06f;
}
</style>