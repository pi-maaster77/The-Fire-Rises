<!-- devtools/web-ui/src/components/layout/LeftSidePanel.vue -->

<script setup lang="ts">
import { ref } from 'vue';
import { load_map_image } from '../../wasm/map_editor.js';
import { useMapStore } from '../../stores/map.js';
import SelectProvince from '../domain/SelectProvince.vue';

const mapStore = useMapStore();
const isLoading = ref(false);

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
    const bytes = new Uint8Array(imageData.data.buffer);

    load_map_image(bytes, img.width, img.height);

    isLoading.value = false;
    URL.revokeObjectURL(img.src);
  };

  img.src = URL.createObjectURL(file);
};
</script>

<template>
  <div class="panel left">
    <h3>Map Editor</h3>

    <input type="file" accept="image/png" @change="handleFileUpload">

    <div v-if="isLoading">Cargando...</div>

    <div v-if="mapStore.selectedProvince">
      <p>ID: {{ mapStore.selectedProvince.id }}</p>
    </div>

    <button class="btn-export" @click="mapStore.exportMap()" :disabled="isLoading">
      Exportar mapa
    </button>

    <button
      class="btn-download"
      v-if="mapStore.exportedMap"
      @click="mapStore.downloadExportedMap()"
    >
      Descargar JSON
    </button>

    <SelectProvince v-if="mapStore.provinces"/>
  </div>
</template>

<style>

.left.panel{
	top: 0;
	left: 0;
}

</style>