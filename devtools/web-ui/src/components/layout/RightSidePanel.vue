<!-- devtools/web-ui/src/components/layout/RightSidePanel.vue -->

<script setup lang="ts">
import StateCreator from '../domain/StateCreator.vue';
import StateSelector from '../domain/StateSelector.vue';

</script>

<!-- devtools/web-ui/src/components/layout/RightSidePanel.vue -->

<template>
  <div class="panel right">
    <h3>Herramientas</h3>
		<StateSelector />
		<StateCreator/>
  </div>
</template>

<style>
.panel.right{
	top: 0;
	right: 0;
}

</style>