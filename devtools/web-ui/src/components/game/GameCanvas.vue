<!-- devtools/web-ui/src/components/game/GameCanvas.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import init from '../../wasm/map_editor.js';
import wasmUrl from '../../wasm/map_editor_bg.wasm?url';
import { setupBridge } from '../../wasm/bridge.js';

const canvasRef = ref<HTMLCanvasElement | null>(null);

onMounted(async () => {
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
      console.error(error);
    }
  }
});
</script>

<template>
  <canvas id="canvas" ref="canvasRef"></canvas>
</template>

<style>
#canvas {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
	z-index: -1;
}
</style>