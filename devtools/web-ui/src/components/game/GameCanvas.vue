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
  top: 0;
  left: 20vw;
  width: 80vw;  /* Cambiado de 100vw a 80vw */
  height: 100vh;
  display: block;
}
</style>