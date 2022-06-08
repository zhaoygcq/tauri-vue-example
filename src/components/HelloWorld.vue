<template>
  <div class="hello">
    <p>{{ msg }}</p>
    <input v-model="inputData" @change="handleChange"/>
    <p v-if="response">response: {{response}}</p>
  </div>
</template>

<script setup>
import { defineProps, ref } from "vue";
import { invoke } from '@tauri-apps/api/tauri'

defineProps(['msg']);

let inputData = ref("");
let response = ref("");

const handleChange = async () => {
  try {
    // 与主进程通信
    const res = await invoke('hello_world_test', {
      event: inputData.value || "none"
    });
    response.value = res;
  } catch(err) {
    console.log(err, "=====error");
  }
}


</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
input {
  border-color: aqua;
}
</style>
