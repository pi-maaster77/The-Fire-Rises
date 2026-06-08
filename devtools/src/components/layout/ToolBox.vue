<!--
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/components/layout/ToolBox.vue * 
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
  <div class="toolbox">
    <BrushSystem />
    <StateManagement />
  </div>
</template>

<script setup lang="ts">
import StateManagement from '../domain/management/StateManagement.vue'
import BrushSystem from '../domain/systems/BrushSystem.vue'
</script>

<style scoped>
.toolbox {
  position: absolute;
  top: 0;
  left: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
  z-index: 10;
  width: 20vw;
  height: 100%;
  color: white;
  background-color: black;
}
</style>
