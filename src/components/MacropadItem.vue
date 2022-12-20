<script setup lang="ts">
import {Ref, ref} from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import router from "../routes";
import mappings from "../mappings";

const props = defineProps(['apps'])
const emit = defineEmits(['refresh']);

type MappingItem = {
  [key: number]: string
}

function getMapping(number: number) {
  return mappings[number];
}

async function clear(number: number) {
  await invoke('set_mapping', {'mapping': { 'key': +number, 'value': "" }});
  emit('refresh');
}

async function manual(number: number) {
  let app = prompt("Manually specify the application name:", props.apps[number]);
  if (app == null || app == "") {
    return;
  }
  await invoke('set_mapping', {'mapping': { 'key': +number, 'value': app }});
  emit('refresh');
}

async function setMapping(number: number) {
  await router.push({'name': 'processes', 'params': { "id": number }});

  /*let app = "";

  let app = prompt("Set the application name.", apps.value[number]);
  console.log(app);
  if (app == null || app == "") {
    return;
  }
  await invoke('set_mapping', {'mapping': { 'key': +number, 'value': app }});
  await get_apps();*/
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
</script>

<template>
  <ul v-if="apps !== null">
    <li v-for="(val, key) in apps" :key="key">
      <div @click="setMapping(key)" class="keyboard-key">
        <span @click.stop="manual(key)" class="manual"><i class="fa fa-edit"></i></span>
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

  div.keyboard-key span.manual {
    position: absolute;
    top: 0;
    left: 4px;
    font-size: 9px;
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
    word-break: break-all;
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
