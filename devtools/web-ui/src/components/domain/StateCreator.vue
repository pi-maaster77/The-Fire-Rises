<!-- devtools/web-ui/src/components/domain/StateCreator.vue -->

<template>
  <label for="name">nombre</label>
  <input type="text" v-model="name">

  <label for="id">id</label>
  <input type="text" v-model="id">

  <label for="region">región</label>

  <button @click="create">+</button>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useStatesStore } from '../../stores/states';

const name = ref('')
const id = ref('')
const selectedRegionId = ref('')

const statesStore = useStatesStore();

function create(){
  statesStore.registerState(id.value, name.value, selectedRegionId.value || null)
  name.value = ''
  id.value = ''
  selectedRegionId.value = ''
}
</script>