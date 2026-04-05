<!-- devtools/web-ui/src/components/domain/RegionPanel.vue -->

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRegionStore } from '../../stores/region';
import { useStatesStore } from '../../stores/states';
import { set_active_region, create_region } from '../../wasm/map_editor.js';

const regionStore = useRegionStore();
const statesStore = useStatesStore();

const regionName = ref("")
const regionId = ref("")
const regionColor = ref("#000000")

const selectedStateId = ref("")


function saveRegion(){
	regionStore.createRegion(regionId.value, regionName.value, regionColor.value)
	create_region(regionId.value, regionName.value)
	regionId.value = '';
	regionName.value = '';
	regionColor.value = '#000000';
}

function assignState(){
	regionStore.assignStateToRegion(selectedStateId.value)
	selectedStateId.value = ""
}

watch(() => regionStore.activeRegionId, (newId) => {
  set_active_region(newId);
});

function getTextColor(hex: string): string {
  // quitar #
  const c = hex.replace('#', '');

  const r = parseInt(c.substring(0, 2), 16);
  const g = parseInt(c.substring(2, 4), 16);
  const b = parseInt(c.substring(4, 6), 16);

  // fórmula de luminancia
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b);

  return luminance > 186 ? '#000000' : '#FFFFFF';
}

</script>

<template>
	<input type="text" placeholder="Nombre De Region" v-model="regionName">
	<input type="text" placeholder="ID De Region" v-model="regionId">
	<input type="color" v-model="regionColor">
	<button @click="saveRegion">+</button>

	<select name="" id="" v-model="regionStore.activeRegionId">
		<option v-for="region in regionStore.regions" :key="region.id" :value="region.id">
			<span :style="{
				color: getTextColor(region.color),
				background: region.color
			}">
			{{region.name}}
			</span>
		</option>
	</select>

	<ul>
		<li v-for="stateId in regionStore.activeRegion?.stateIds" :key="stateId">{{ stateId }}{{ statesStore.getStateById(stateId)?.name }}</li>
	</ul>

	<select name="" id="" v-model="selectedStateId">
		<option v-for="state in statesStore.states" :key="state.id" :value="state.id">{{ state.name }}</option>
	</select>
	<button @click="assignState">+</button>
</template>