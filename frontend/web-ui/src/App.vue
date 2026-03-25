<!-- frontend/web-ui/src/App.vue -->

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import init from './wasm/map_engine.js';
import wasmUrl from './wasm/map_engine_bg.wasm?url';
import SelectedProvince from './components/domain/SelectedProvince.vue';
import { setupBridge } from './wasm/bridge.js';


const canvasRef = ref<HTMLCanvasElement | null>(null);

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
</script>

<template>
  <div class="game-wrapper">
    <div class="ui-layer">
      <div class="panel">
				<SelectedProvince/>
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
}
</style>