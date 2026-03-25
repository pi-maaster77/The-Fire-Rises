<!-- frontend/web-ui/src/components/domain/SelectedProvince.vue -->

<template>
  <div class="interface">
    <div class="panel">
      <h3>Provincias</h3>
      <ul>
        <li v-for="provinceId in mapStore.provinces" :key="provinceId">
          {{ provinceId }}
        </li>
      </ul>
    </div>
    <div class="panel" v-if="mapStore.selectedProvince">
      <h3>Información de Territorio</h3>
      <p><strong>Provincia ID:</strong> {{ mapStore.selectedProvince.id }}</p>
      <p><strong>Centro:</strong> {{ mapStore.selectedProvince.center }}</p>
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