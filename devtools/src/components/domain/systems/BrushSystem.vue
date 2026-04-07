<!-- devtools/src/components/domain/systems/BrushSystem.vue -->

<script setup lang="ts">
import { useStateStore } from '@/stores/states'
import { useEngineStore } from '@/stores/engine' // <--- Nueva store
import { computed } from 'vue'

const stateStore = useStateStore()
const engineStore = useEngineStore()

const selectedStateId = computed({
  get: () => stateStore.selectedState?.id,
  set: (value: string) => stateStore.selectState(value),
})

function toggleBrush() {
  if (!stateStore.selectedState) return

  stateStore.isBrushing = !stateStore.isBrushing

  // Acceso directo y limpio
  const engine = engineStore.instance
  if (!engine) return

  engine.setCameraControl(!stateStore.isBrushing)

  if (stateStore.isBrushing) {
    engine.onBrushStroke = (index) => {
      const color = parseInt(stateStore.selectedState!.color.replace('#', '0x'))
      engine.updateProvinceVisual(index, color)
      stateStore.assignProvinceToState(stateStore.selectedState!.id, index)
    }
  } else {
    engine.onBrushStroke = undefined
  }
}
</script>

<template>
  <div class="brush-tool">
    <select v-model="selectedStateId">
      <option disabled value="">Seleccionar Estado</option>
      <option v-for="state in stateStore.states" :key="state.id" :value="state.id">
        {{ state.name }}
      </option>
    </select>

    <button @click="toggleBrush" :class="{ 'btn-active': stateStore.isBrushing }">
      {{ stateStore.isBrushing ? '🖌️ Pincel Activo' : '🖐️ Modo Cámara' }}
    </button>
  </div>
</template>

<style scoped>
.brush-tool {
  background: rgba(0, 0, 0, 0.8);
  padding: 15px;
  border-radius: 8px;
  pointer-events: auto;
}
.btn-active {
  background-color: #ccff00;
  color: black;
  font-weight: bold;
}
</style>
