<!-- devtools/web-ui/src/components/domain/RegionPanel.vue -->

<template>
  <div class="region-panel">
    <h3>Gestión de Regiones</h3>
    
    <button @click="store.createRegion('Nueva Región', '#ff0000')">
      + Crear Región
    </button>

    <div class="region-list">
      <div 
        v-for="region in store.regions" 
        :key="region.id"
        :class="['region-item', { active: store.activeRegionId === region.id }]"
        @click="store.setActiveRegion(region.id)"
      >
        <span class="color-dot" :style="{ backgroundColor: region.color }"></span>
        {{ region.name }} ({{ region.stateIds.length }} estados)
      </div>
    </div>

    <button v-if="store.activeRegionId" @click="store.setActiveRegion(null)">
      Desactivar Modo Asignación
    </button>
  </div>
</template>

<script setup lang="ts">
import { useRegionStore } from '../../stores/region';
const store = useRegionStore();
</script>

<style scoped>
.region-item { padding: 8px; cursor: pointer; border: 1px solid #333; margin-top: 4px; }
.region-item.active { border-color: #00ff00; background: #1a3a1a; }
.color-dot { display: inline-block; width: 12px; height: 12px; border-radius: 50%; }
</style>