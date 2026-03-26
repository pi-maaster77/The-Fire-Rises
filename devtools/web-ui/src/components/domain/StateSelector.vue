<!-- devtools/web-ui/src/components/domain/StateSelector.vue -->

<script setup lang="ts">
import { ref } from 'vue';
import { useStatesStore } from '../../stores/states';


const statesStore = useStatesStore();

const state_id = ref('')

</script>

<template>
	<select v-model="state_id">
		<option v-for="state in statesStore.states" :key="state.id" :value="state.id">{{ state.name }}</option>
		<option v-if="!statesStore.states.length">No hay estados cargadas</option>
	</select>

</template>