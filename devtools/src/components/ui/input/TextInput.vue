<!-- devtools/src/components/ui/input/TextInput.vue -->

<template>
  <div class="text-input">
    <input
      type="text"
      :value="modelValue"
      @input="update"
      @blur="blur"
      class="form-control"
      :disabled="disabled"
    />
    <label for="text-input">
      <slot></slot>
    </label>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: string
  disabled?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

function update(event: Event) {
  const value = (event.target as HTMLInputElement).value
  emit('update:modelValue', value)
}

function blur(event: Event) {
  const value = (event.target as HTMLInputElement).value.trim()
  emit('update:modelValue', value)
}
</script>

<style scoped></style>
