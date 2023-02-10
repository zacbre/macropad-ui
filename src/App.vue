<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
  <div class="container">
    <RouterView />
    <div class="copyright">
      <p>Macropad-UI 1.0.2 by <a href="https://github.com/zacbre" target="_blank">zacbre</a></p>
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
.copyright p {
  margin: 10px 0 0 0;
}
.copyright {
  position: absolute;
  bottom: 20px;
  left: 0;
  right: 0;
}
</style>
