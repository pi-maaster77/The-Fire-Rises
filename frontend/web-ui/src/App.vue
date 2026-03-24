<!-- frontend/web-ui/src/App.vue -->

<script setup lang="ts">
import { useGameStore } from './stores/game.ts';
const gameStore = useGameStore();

const increment = () => {
  gameStore.updateFromUI(gameStore.counter + 1);
};

</script>

<template>
  <div class="counter-card">
    <h3>Sincronización de Datos</h3>
    <p>Valor Actual: <strong>{{ gameStore.counter }}</strong></p>
    <button @click="increment">Incrementar desde Vue</button>
  </div>
</template>

<style scoped>
.counter-card {
  background: rgba(20, 20, 25, 0.8);
  padding: 20px;
  border-radius: 8px;
  border: 1px solid #444;
  color: white;
  pointer-events: auto; /* Para que el botón sea clickeable sobre el mapa */
}
</style>