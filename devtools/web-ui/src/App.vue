<!-- devtools/web-ui/src/App.vue -->

<script setup lang="ts">
import LeftPanel from './components/layout/LeftSidePanel.vue';
import RightPanel from './components/layout/RightSidePanel.vue';
import GameCanvas from './components/game/GameCanvas.vue';
import './style.css'
</script>

<template>
  <div class="app">
    <LeftPanel class="left"/>
    <RightPanel class="right"/>
    <GameCanvas class="canvas"/>
  </div>
</template>
