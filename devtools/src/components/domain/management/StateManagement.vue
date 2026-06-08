<!--
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/components/domain/management/StateManagement.vue * 
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
  <table>
    <thead>
      <tr>
        <th>Nombre</th>
        <th>ID</th>
        <th>Color</th>
        <th>Botones</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="state in stateStore.states" :key="state.id">
        <td>{{ state.name }}</td>
        <td>{{ state.id }}</td>
        <td :style="{ backgroundColor: state.color }" class="color-cell"></td>
        <td>
          <button @click="deleteState(state.id)">Eliminar</button>
        </td>
        <td>
          <button @click="editState(state.id)">Editar</button>
        </td>
      </tr>
      <tr>
        <td><TextInput v-model="stateName" placeholder="Nombre del estado" /></td>
        <td><TextInput v-model="stateId" placeholder="ID del estado" /></td>
        <td><ColorInput v-model="stateColor" /></td>
        <td>
          <button v-if="editing" @click="confirmEdit">Confirmar</button>
          <button v-else @click="addState">Agregar</button>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script setup lang="ts">
import TextInput from '@/components/ui/input/TextInput.vue'
import ColorInput from '@/components/ui/input/ColorInput.vue'
import { ref } from 'vue'
import { useStateStore } from '@/stores/states'
import type { State } from '@/types/Map'

const editing = ref<null | string>(null)
const stateName = ref('')
const stateId = ref('')
const stateColor = ref(randomColor())

const stateStore = useStateStore()

function randomColor() {
  const letters = '0123456789ABCDEF'
  let color = '#'
  for (let i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)]
  }
  return color
}

function addState() {
  if (!stateName.value || !stateId.value || !stateColor.value) {
    alert('Por favor, completa todos los campos para agregar el estado.')
    return
  }
  stateStore.add({
    name: stateName.value,
    id: stateId.value,
    color: stateColor.value,
  } as State)
  stateName.value = ''
  stateId.value = ''
  stateColor.value = randomColor()
}
function deleteState(id: string) {
  stateStore.remove(id)
}
function editState(id: string) {
  editing.value = id
  const state = stateStore.get(id)
  if (!state) return
  stateName.value = state.name
  stateId.value = state.id
  stateColor.value = state.color
}

function confirmEdit() {
  if (!stateName.value || !stateId.value || !stateColor.value) {
    alert('Por favor, completa todos los campos para editar el estado.')
    return
  }
  if (!editing.value) return
  stateStore.update(editing.value, {
    name: stateName.value,
    id: stateId.value,
    color: stateColor.value,
  } as State)
  editing.value = null
  stateName.value = ''
  stateId.value = ''
  stateColor.value = randomColor()
}
</script>

<style scoped></style>
