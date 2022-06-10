<template>
  <div class="hello">
    <p>{{ title }}</p>
    <input v-model="inputData" @change="handleChange"/>
    <p v-if="response">response: {{response}}</p>
  </div>
</template>

<script setup>
import { defineProps, ref, onMounted } from "vue";
import { invoke } from '@tauri-apps/api/tauri'
import { listen, emit,  } from "@tauri-apps/api/event"

let props = defineProps(['msg']);

let title = ref(props.msg);

let inputData = ref("");
let response = ref("");

const handleChange = async () => {
  try {
    // 与主进程通信
    // 使用Event进行消息通信
    await listen('clickRes', event => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      console.log(event, "=======event=======");
      response.value = event.payload;
    })

    emit('clickReq', {
      theMessage: 'Tauri is awesome111111111111!'
    })
  } catch(err) {
    console.log(err, "=====error");
  }
}

onMounted(async () => {
  try {
    // 使用Invoke，类似于网络请求的方式进行通信
    let res = await invoke("my_custom_command", {
      event: "Kangkang"
    });

    title.value = res;
    console.log(res, "======onMounted======");
  } catch(e) {
    console.log("error in mounted=======", e);
  }
})

</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
input {
  border-color: aqua;
}
</style>
