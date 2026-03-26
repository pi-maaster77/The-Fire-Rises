<!-- devtools/web-ui/src/components/domain/StateSelector.vue -->

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useStatesStore } from '../../stores/states';
import { update_brush_settings } from '../../wasm/map_editor.js';

const statesStore = useStatesStore();
const selectedStateId = ref('');
const isPainting = ref(false);

// Función principal para activar/desactivar el pincel
const togglePaintMode = () => {
  if (!selectedStateId.value && !isPainting.value) {
    alert("Selecciona un estado primero");
    return;
  }
  
  isPainting.value = !isPainting.value;
  
  // Llamada al Wasm: (activo, id_del_estado)
  update_brush_settings(isPainting.value, selectedStateId.value);
};

// Si el usuario cambia el estado en el select, actualizamos Rust en tiempo real
watch(selectedStateId, (newId) => {
  if (isPainting.value) {
    update_brush_settings(true, newId);
  }
});
</script>

<template>
  <div class="state-editor-controls">
    <div class="field">
      <label>Estado Activo:</label>
      <select v-model="selectedStateId" :disabled="isPainting">
        <option value="">-- Seleccionar --</option>
        <option v-for="state in statesStore.states" :key="state.id" :value="state.id">
          {{ state.name }} ({{ state.id }})
        </option>
      </select>
    </div>

    <button 
      @click="togglePaintMode"
      :class="['btn-paint', { 'is-active': isPainting }]"
    >
      <span v-if="!isPainting">🖌️ Activar Pincel</span>
      <span v-else>🛑 Detener Pincel</span>
    </button>

    <p v-if="isPainting" class="hint">
      Mantén el <strong>Click Izquierdo</strong> sobre el mapa para asignar provincias.
    </p>
  </div>
</template>

<style scoped>
.state-editor-controls {
  padding: 1rem;
  background: #222;
  border-radius: 8px;
  color: white;
}

.field {
  margin-bottom: 1rem;
}

select {
  width: 100%;
  padding: 0.5rem;
  background: #333;
  color: white;
  border: 1px solid #444;
}

.btn-paint {
  width: 100%;
  padding: 0.75rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  transition: all 0.2s;
  background: #444;
  color: #ccc;
}

.btn-paint.is-active {
  background: #e67e22; /* Naranja para resaltar que está grabando */
  color: white;
  box-shadow: 0 0 10px rgba(230, 126, 34, 0.5);
}

.hint {
  margin-top: 0.5rem;
  font-size: 0.85rem;
  color: #aaa;
}
</style>