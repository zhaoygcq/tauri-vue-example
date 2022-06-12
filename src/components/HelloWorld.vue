<template>
  <div class="hello">
    <p>{{ title }}</p>
    <input v-model="inputData" @change="handleInputValChange"/>
    <p v-if="response">response: {{response}}</p>
    <span class="history" v-if="response" @click="getHistory">查看输入历史：</span>
    <ul>
      <li v-for="(history, index) in histories" :key="index">{{ history }}</li>
    </ul>
  </div>
</template>

<script setup>
import { defineProps, ref, onMounted } from "vue";
import { invoke } from '@tauri-apps/api/tauri'
import { emit,  } from "@tauri-apps/api/event"

let props = defineProps(['msg']);

let title = ref(props.msg);

let inputData = ref("");
let response = ref("");
let histories = ref([]);

const handleInputValChange = async () => {
  try {
    // 与主进程通信
    let res = await invoke("store_msg", {
      event: inputData.value
    })
    console.log(res, "======res");
    response.value = res;

    // 使用Event进行消息通信, 触发主进程监听的事件
    emit('clickReq', {
      theMessage: inputData.value
    })
  } catch(err) {
    console.log(err, "=====error");
  }
}

const getHistory = async () => {
  try {
    let res = await invoke("get_history");
    histories.value = res;
    console.log(res);
  } catch(err) {
    console.log(err);
  }
}

onMounted(async () => {
  try {
    // 使用Invoke，类似于网络请求的方式进行通信
    let res = await invoke("my_custom_command", {
      event: "Kangkang"
    });

    title.value = res;
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

.hello {
  margin: 0 auto;
  width: 400px;
  display: flex;
  flex-direction: column;
}

.history {
  cursor: pointer;
  align-self: flex-start;
} 

ul {
  margin: 0;
  padding: 0;
  list-style: none;
}

ul li {
  text-align: left;
}
</style>
