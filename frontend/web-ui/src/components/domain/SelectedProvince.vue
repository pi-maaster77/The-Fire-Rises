<!-- frontend/web-ui/src/components/domain/SelectedProvince.vue -->

<template>
  <div class="interface" v-if="mapStore.selectedProvince">
    <div class="panel">
      <h3>Información de Territorio</h3>
      <p><strong>Provincia ID:</strong> {{ mapStore.selectedProvince.id }}</p>
      <p><strong>Coordenadas:</strong> {{ mapStore.selectedProvince.coords }}</p>
      <hr />
      <p><strong>Estado:</strong> {{ mapStore.selectedState.name }}</p>
      <p><strong>Estado ID:</strong> {{ mapStore.selectedState.id }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMapStore } from '../../stores/map';
const mapStore = useMapStore();
</script>