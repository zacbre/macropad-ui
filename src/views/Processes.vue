<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {Ref, ref, UnwrapRef} from "vue";
import router from "../routes";
import mappings from "../mappings";

const process_list: Ref<UnwrapRef<string[]>> = ref([]);

const props = defineProps(['id'])

async function getProcesses() {
  process_list.value = await invoke('get_process_list');
}
async function pushRouter(id: number, process: string) {
  if (process.indexOf(".") !== -1) {
    process = process.split(".")[0];
  }
  await invoke('set_mapping', {'mapping': { 'key': +id, 'value': process.toLowerCase() }});
  await goToHome();
}
async function goToHome() {
  await router.push({'name': 'main'});
}
getProcesses();
</script>

<template>
  <h4>Select a process to audio map to {{ mappings[props.id]}}, or go <router-link to="/">back.</router-link></h4>
  <table class="styled-table" v-if="process_list.length > 0">
    <thead>
      <tr>
        <td>Name</td>
        <td>Select</td>
      </tr>
    </thead>
    <tbody>
      <tr v-for="process in process_list" :key="process">
        <td>{{ process }}</td>
        <td><button @click="pushRouter(props.id, process)">Select</button></td>
      </tr>
    </tbody>
  </table>
  <p v-else>Loading process list...</p>
</template>

<style scoped>
ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}
</style>