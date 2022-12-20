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

async function getConnected() {
  connected.value = await invoke('get_connected_state');
}
getConnected()
setInterval(getConnected, 2000);
</script>

<template>
  <div>
    <p :class="connected === true ? 'green' : 'red'">{{ connectedState }}</p>
  </div>
</template>

<style scoped>
  div {
    position: absolute;
    top: 0;
    right: 25px;
  }

  p.green {
    color: green;
  }

  p.red {
    color: red;
  }
</style>