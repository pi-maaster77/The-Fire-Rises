<!-- devtools/src/components/layout/ToolBox.vue -->

<template>
  <div class="toolbox">
    <BrushSystem />
    <StateManagement />
  </div>
</template>

<script setup lang="ts">
import StateManagement from '../domain/management/StateManagement.vue'
import BrushSystem from '../domain/systems/BrushSystem.vue'
</script>

<style scoped>
.toolbox {
  position: absolute;
  top: 0;
  left: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
  z-index: 10;
  width: 20vw;
  height: 100%;
  color: white;
  background-color: black;
}
</style>
