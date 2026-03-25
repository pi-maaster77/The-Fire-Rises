<!-- frontend/web-ui/src/App.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import init from './wasm/map_engine.js';
import wasmUrl from './wasm/map_engine_bg.wasm?url';

const canvasRef = ref<HTMLCanvasElement | null>(null);

onMounted(async () => {
  // 1. Forzamos el tamaño del buffer ANTES de que Bevy lo toque
  if (canvasRef.value) {
    const canvas = canvasRef.value;
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
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
  <div class="bevy-frame">
    <canvas 
      id="canvas" 
      ref="canvasRef"
    ></canvas>
  </div>
</template>

<style>
.bevy-frame {
  width: 100vw;
  height: 100vh;
  position: fixed;
  top: 0;
  left: 0;
  background: #111;
}

#canvas {
  width: 100% !important;
  height: 100% !important;
  display: block;
  outline: none;
}
</style>