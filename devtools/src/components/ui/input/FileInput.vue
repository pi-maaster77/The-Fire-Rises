<!-- devtools/src/components/ui/input/FileInput.vue -->

<template>
  <div class="file-input">
    <!-- Input real oculto -->
    <input ref="inputRef" type="file" accept=".json" class="hidden-input" @change="onFileUpload" />

    <!-- Label custom (usa slot) -->
    <label
      class="custom-button"
      @click="triggerFile"
      :data-file-name="fileName || 'Ningún archivo seleccionado'"
    >
      <slot> ⬆️ Subir archivo </slot>
    </label>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits(['change'])

const inputRef = ref<HTMLInputElement | null>(null)
const fileName = ref<string | null>(null)

function triggerFile() {
  inputRef.value?.click()
}

function onFileUpload(event: Event) {
  const input = event.target as HTMLInputElement

  if (input.files && input.files.length > 0) {
    fileName.value = input.files[0]?.name || null
  }

  emit('change', event)
}
</script>

<style scoped>
.hidden-input {
  display: none;
}

.custom-button {
  display: inline-block;
  padding: 10px 16px;
  background-color: #2c3e50;
  color: white;
  border-radius: 8px;
  cursor: pointer;
  position: relative;
}

/* Tooltip en hover */
.custom-button:hover::after {
  content: attr(data-file-name);
  position: absolute;
  bottom: -28px;
  left: 0;
  background: black;
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
}
</style>
