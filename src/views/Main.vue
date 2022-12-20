<script setup lang="ts">
import Connected from '../components/Connected.vue';
import MacropadItem from '../components/MacropadItem.vue';
import {invoke} from "@tauri-apps/api/tauri";
import {ref, Ref} from "vue";

type ApplicationItem = {
  [key: number]: string
};

const apps: Ref<ApplicationItem> = ref({});

async function getApps() {
  apps.value = await invoke('get_apps');
}
getApps();
</script>

<template>
  <h1>Macropad-UI</h1>
  <Connected />
  <MacropadItem :apps="apps" @refresh="() => getApps()"/>
</template>

<style scoped>

</style>