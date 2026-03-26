<!-- devtools/web-ui/src/components/domain/StateCreator.vue -->

<template>
	<label for="name">nombre</label>
	<input type="text" v-model="name">
	
	<label for="name">id</label>
	<input type="text" v-model="id">
	<button @click="create">+</button>

</template>

<script setup lang="ts">
	import { ref } from 'vue'
import { useStatesStore } from '../../stores/states';
	const name = ref('')
	const id = ref('')

	const statesStore = useStatesStore();

	function create(){
		statesStore.addState(name.value, id.value)
		name.value = ''
		id.value = ''
	}
</script>