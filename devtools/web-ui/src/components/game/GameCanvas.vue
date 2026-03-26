<!-- devtools/web-ui/src/components/game/GameCanvas.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import init from '../../wasm/map_editor.js';
import wasmUrl from '../../wasm/map_editor_bg.wasm?url';
import { setupBridge } from '../../wasm/bridge.js';

const canvasRef = ref<HTMLCanvasElement | null>(null);

onMounted(async () => {
	const resizeCanvas = () => {
    if (canvasRef.value) {
      const canvas = canvasRef.value;
      // Usamos getBoundingClientRect para obtener el tamaño real en píxeles
      const rect = canvas.getBoundingClientRect();
      canvas.width = rect.width;
      canvas.height = rect.height;
    }
  };
  
  window.addEventListener('resize', resizeCanvas);
  resizeCanvas();


  setupBridge();

  try {
    await new Promise(r => requestAnimationFrame(r));
    await init({ module_or_path: wasmUrl });
  } catch (error: any) {
    if (!error.toString().includes("exceptions for control flow")) {
      console.error(error);
    }
  }
});
</script>

<template>
  <canvas id="canvas" ref="canvasRef"></canvas>
</template>

<style scoped>
#canvas {
	position: absolute;
  /* Si el sidebar mide 20vw, el canvas debe medir el resto */
  left: 20vw !important;
	width: calc(100vw - 40vw) !important; 

  height: 100vh !important; /* Probemos con tus 200px */
  display: block;
  background-color: #111; /* Para ver dónde termina el canvas */
	z-index: 999;
}
</style>