<!--
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/components/layout/PopupWindow.vue * 
 * Project: The Fire Rises                                                    * 
 * Created Date: Mo Jun 2026                                                  * 
 * Author: pi-maaster77 (pimaaster1337@gmail.com)                             * 
 * -----                                                                      * 
 * Last Modified: Mon Jun 08 2026                                             * 
 * Modified By: pi-maaster77                                                  * 
 * -----                                                                      * 
 * Copyright (c) projectCreationYear-2026 The Fire Rises                      * 
 * -----                                                                      * 
 * License: GNU Affero General Public License v3.0 (AGPL-3.0)                 * 
 * This program is free software: you can redistribute it and/or modify       * 
 * it under the terms of the GNU Affero General Public License as             * 
 * published by the Free Software Foundation, either version 3 of the         * 
 * License, or (at your option) any later version.                            * 
 * -----                                                                      * 
 * HISTORY:                                                                   * 
 * Date      	By	Comments                                                    * 
 * ----------	---	---------------------------------------------------------   * 
-->

<template>
  <div v-if="isOpen" class="popup-overlay" @click.self="close">
    <div class="popup-window">
      <header class="popup-header">
        <h3>{{ title }}</h3>
        <button class="close-btn" @click="close">×</button>
      </header>

      <main class="popup-content">
        <slot></slot>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  isOpen: boolean
  title: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

function close() {
  emit('close')
}
</script>

<style scoped>
.popup-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}
.popup-window {
  background: #222; /* Ajustalo al estilo oscuro de tus devtools */
  border: 1px solid #444;
  border-radius: 6px;
  min-width: 400px;
  max-width: 90vw;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
}
.popup-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 15px;
  border-bottom: 1px solid #444;
}
.popup-content {
  padding: 15px;
  max-height: 70vh;
  overflow-y: auto;
}
.close-btn {
  background: transparent;
  border: none;
  color: #fff;
  font-size: 1.5rem;
  cursor: pointer;
}
</style>
