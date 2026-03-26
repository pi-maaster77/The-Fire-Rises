<!-- devtools/web-ui/src/components/domain/StateCreator.vue -->

<template>
	<input type="text" v-model="name">
	<input type="text" v-model="id">
	<button @click="create">+</button>

</template>

<script setup lang="ts">
	import { ref } from 'vue'
	const name = ref('')
	const id = ref('')

	function create(){
		
	}
</script>