<script setup lang="ts">
import {Ref, ref} from "vue";
import { invoke } from "@tauri-apps/api/tauri";

type ApplicationItem = {
  [key: number]: string
};

type MappingItem = {
  [key: number]: string
}

const apps: Ref<ApplicationItem> = ref({});
const mappings: Ref<MappingItem> = ref({
  0x00C0: "FN0",
  0x00C1: "FN1",
  0x00C2: "FN2",
  0x00C3: "FN3",
  0x00C4: "FN4",
  0x00C5: "FN5",
  0x00C6: "FN6",
  0x00C7: "FN7",
  0x00C8: "FN8",
  0x00C9: "FN9",
  0x00CA: "FN10",
  0x00CB: "FN11",
  0x00CC: "FN12",
  0x00CD: "FN13",
  0x00CE: "FN14",
  0x00CF: "FN15",
  0x00D0: "FN16",
  0x00D1: "FN17",
  0x00D2: "FN18",
  0x00D3: "FN19",
  0x00D4: "FN20",
  0x00D5: "FN21",
  0x00D6: "FN22",
  0x00D7: "FN23",
  0x00D8: "FN24",
  0x00D9: "FN25",
  0x00DA: "FN26",
  0x00DB: "FN27",
  0x00DC: "FN28",
  0x00DD: "FN29",
  0x00DE: "FN30",
  0x00DF: "FN31"
});

async function get_apps() {
  apps.value = await invoke('get_apps');
}

function getMapping(number: number) {
  return mappings.value[number];
}

async function clear(number: number) {
  await invoke('set_mapping', {'mapping': { 'key': +number, 'value': "" }});
  await get_apps();
}

async function setMapping(number: number) {
  if (apps.value == null) {
    return;
  }

  let app = prompt("Set the application name.", apps.value[number]);
  console.log(app);
  if (app == null || app == "") {
    return;
  }
  await invoke('set_mapping', {'mapping': { 'key': +number, 'value': app }});
  await get_apps();
}

function determineClass(val: string) {
  if (val == null || val == '') {
    return "unset";
  }
  if (val.length > 20) {
    return "too-long";
  }

  return "value";
}

function determineValue(val: string) {
  if (val == null || val == "") {
    return "unset";
  }
  if (val.length > 20) {
    return val.slice(0, 20)+"...";
  }
  return val;
}

get_apps();
</script>

<template>
  <ul v-if="apps !== null">
    <li v-for="(val, key) in apps" :key="key">
      <div @click="setMapping(key)" class="keyboard-key">
        <span @click.stop="clear(key)" class="clear"><i class="fa fa-x"></i></span>
        <p class="title">{{ getMapping(key) }}</p>
        <p :class="determineClass(val)">{{ determineValue(val) }}<span class="tooltiptext" v-if="val.length > 20">{{ val }}</span></p>
      </div>
    </li>
  </ul>
</template>

<style scoped>
  ul {
    list-style-type: none;
    text-align: left;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    align-content: stretch;
    justify-content: center;
    margin: 0;
    padding: 0;
  }
  div.keyboard-key {
    border: 1px solid #222;
    border-radius: 5px;
    width: 100px;
    height: 100px;
    text-align: center;
    margin: 5px;
    cursor: pointer;
    transition: 0.5s ease all;
    position: relative;
  }

  div.keyboard-key span.clear {
    position: absolute;
    top: 0;
    right: 4px;
    font-size: 9px;
  }

  div.keyboard-key:hover {
    background: #444;
  }

  div.keyboard-key p {
    margin: 10px 0;
  }

  div.keyboard-key p.title {
    font-weight: 700;
    color: #888;
    margin-bottom: 5px;
  }

  div.keyboard-key p.unset {
    color: #555;
  }

  div.keyboard-key p.too-long {
    position: relative;
  }

  div.keyboard-key p.too-long .tooltiptext {
    visibility: hidden;
    width: 120px;
    background-color: #555;
    color: #fff;
    text-align: center;
    border-radius: 6px;
    padding: 5px 0;
    position: absolute;
    z-index: 1;
    bottom: 125%;
    left: 50%;
    margin-left: -60px;
    opacity: 0;
    transition: opacity 0.3s;
  }

  div.keyboard-key p.too-long .tooltiptext::after {
    content: "";
    position: absolute;
    top: 100%;
    left: 50%;
    margin-left: -5px;
    border-width: 5px;
    border-style: solid;
    border-color: #555 transparent transparent transparent;
  }

  div.keyboard-key p.too-long:hover .tooltiptext {
    visibility: visible;
    opacity: 1;
  }
</style>
