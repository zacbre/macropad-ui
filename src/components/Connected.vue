<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {computed, Ref, ref, UnwrapRef} from "vue";

const connected: Ref<UnwrapRef<boolean>> = ref(false);

const connectedState = computed(() => {
  if (connected.value) {
    return "Connected to Macropad!";
  }
  return "Not Connected to Macropad!";
});

async function openWindow() {
  await invoke('open_window', { 'url': '/via/index.html' });
}

async function getConnected() {
  connected.value = await invoke('get_connected_state');
}
getConnected()
setInterval(getConnected, 2000);
</script>

<template>
  <div class="left">
    <p><a @click="openWindow">Open Via</a></p>
  </div>
  <div class="right">
    <p :class="connected === true ? 'green' : 'red'">{{connectedState}}</p>
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
  }

  p.red {
    color: red;
  }

  a {
    cursor: pointer;
  }
</style>