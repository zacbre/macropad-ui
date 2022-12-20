<script setup lang="ts">
import Connected from '../components/Connected.vue';
import MacropadItem from '../components/MacropadItem.vue';
import {invoke} from "@tauri-apps/api/tauri";
import {ref, Ref} from "vue";

type ApplicationItem = {
  [key: number]: string
};

type Settings = {
  proc_list: ApplicationItem,
  setting_item_1: boolean
}

const settings: Ref<Settings | null> = ref(null);

async function getApps() {
  settings.value = await invoke('get_apps');
}
getApps();
</script>

<template>
  <h1>Macropad-UI</h1>
  <Connected />
  <MacropadItem :settings="settings" @refresh="() => getApps()"/>
</template>

<style scoped>

</style>