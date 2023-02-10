<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {computed, Ref, ref, UnwrapRef} from "vue";

const connected: Ref<UnwrapRef<boolean>> = ref(false);
const numberValue: Ref<UnwrapRef<number>> = ref(5);
const connectedState = computed(() => {
  if (connected.value) {
    return "Connected to Macropad!";
  }
  return "Not Connected to Macropad!";
});

async function openWindow() {
  await invoke('open_window', { 'url': '/via/index.html' });
}

async function handleVolumeInc() {
  await invoke('set_increment', { 'vol': numberValue.value });
}

async function getConnected() {
  connected.value = await invoke('get_connected_state');
}
async function getVolumeInc() {
  numberValue.value = await invoke('get_volume_inc');
}
getConnected();
getVolumeInc();
setInterval(getConnected, 2000);
</script>

<template>
  <div class="left">
    <p><a @click="openWindow">Open Via</a></p>
  </div>
  <div class="right">
    <p :class="connected === true ? 'green' : 'red'">{{connectedState}}</p>
    <span style="font-size: 13px;">Volume Increment: </span>
    <input type="number" step="1" min="1" max="10" style="width: 36px;padding: 5px;font-size: 13px;" v-model="numberValue" @change="handleVolumeInc">
  </div>
</template>

<style scoped>
  div.left {
    position: absolute;
    top: 0;
    left: 23px;
  }
  div.right {
    position: absolute;
    top: 0;
    right: 23px;
  }

  p.green {
    color: green;
    margin-bottom: 0;
  }

  p.red {
    color: red;
  }

  a {
    cursor: pointer;
  }
</style>