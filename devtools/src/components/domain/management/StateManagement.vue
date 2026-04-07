<!-- devtools/src/components/domain/management/StateManagement.vue -->

<template>
  <TextInput v-model="stateName" placeholder="Nombre del estado"> Nombre del Estado </TextInput>
  <TextInput v-model="stateId" placeholder="ID del estado"> ID del Estado </TextInput>
  <ColorInput v-model="stateColor"> Color del Estado </ColorInput>
  <button @click="addState">+</button>
</template>

<script setup lang="ts">
import TextInput from '@/components/ui/input/TextInput.vue'
import ColorInput from '@/components/ui/input/ColorInput.vue'
import { ref } from 'vue'
import { useStateStore } from '@/stores/states'
import type { State } from '@/types/Map'

const stateName = ref('')
const stateId = ref('')
const stateColor = ref(randomColor())

const stateStore = useStateStore()

function randomColor() {
  const letters = '0123456789ABCDEF'
  let color = '#'
  for (let i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)]
  }
  return color
}

function addState() {
  console.log('Adding state:', {
    name: stateName.value,
    id: stateId.value,
    color: stateColor.value,
  })
  stateStore.add({
    name: stateName.value,
    id: stateId.value,
    color: stateColor.value,
  } as State)
}
</script>

<style scoped></style>
