<!-- devtools/web-ui/src/components/domain/SelectProvince.vue -->

<script setup lang="ts">
import { computed } from 'vue';
import { useMapStore } from '../../stores/map';


const mapStore = useMapStore();

const province_id = computed({
  get: () => mapStore.selectedProvince?.id ?? "",
  set: (value: string) => {
    mapStore.selectFromUI(value);
  }
});

</script>

<template>
	<select v-model="province_id">
		<option v-for="province in mapStore.provinces" :key="province" :value="province">{{province}}</option>
		<option v-if="!mapStore.provinces.length">No hay provincias cargadas</option>
	</select>

</template>