<template>
  <div class="hello">
    <p>{{ title }}</p>
    <label>输入内容：</label>
    <div class="input-area">
      <input v-model="inputData" @change="handleInputValChange"/>
      <button v-if="inputData" @click="setClipboardContent">复制到剪切板</button>
    </div>
    <label>选择文件</label>
    <input v-model="filePath" @click="selectSourceFile" />
    <p v-if="response" @click="getClipboardContent">response: {{response}}</p>
    <span class="history" v-if="response" @click="getHistory">查看输入历史：</span>
    <ul>
      <li v-for="(history, index) in histories" :key="index">{{ history }}</li>
    </ul>
  </div>
</template>

<script setup>
import { defineProps, ref, onMounted } from "vue";
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
import { Store } from 'tauri-plugin-store-api';
import { readText, writeText } from '@tauri-apps/api/clipboard';
import { sendNotification } from '@tauri-apps/api/notification';
import { register, isRegistered } from '@tauri-apps/api/globalShortcut';

const store = new Store('.settings.dat');

const setStore = async (value) => {
  await store.set('some-key', { value });
  const val = await store.get('some-key');

  console.log(val);
}

let props = defineProps(['msg']);

let title = ref(props.msg);
let filePath = ref("");

let inputData = ref("");
let response = ref("");
let histories = ref([]);

const handleInputValChange = async () => {
  try {
    // 与主进程通信
    await invoke("store_msg", {
      event: inputData.value
    })
    listen("rust-event", (res) => {
      console.log(res, "======res");
      response.value = res && res.payload.data;
    });

    // 使用Event进行消息通信, 触发主进程监听的事件
    emit('js-event', {
      theMessage: inputData.value
    })

    await setStore(inputData.value);
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

const selectSourceFile = async () => {
  try {
    let res = await open({
      title: "请选择文件夹",
      defaultPath: "..",
      directory: false
    })
    filePath.value = res;
    console.log(res, "======file checked=====");
  } catch(e) {
    console.log("error when open file", e);
  }
}

const getClipboardContent = async () => {
  try {
    const clipboardText = await readText();
    console.log(clipboardText, "======clipboard content======");
  } catch(err) {
    console.log("get clipboard error", err);
  }
}

const setClipboardContent = async () => {
  try {
    await writeText(inputData.value);
    sendNotification({ title: 'Copy', body: '复制到剪切板成功' });
  } catch(err) {
    console.log("setClipboardContent error", err);
  }
}

onMounted(async () => {
  try {
    // 使用Invoke，类似于网络请求的方式进行通信
    let res = await invoke("my_custom_command", {
      event: "Kangkang"
    });

    title.value = res;

    let flag = await isRegistered('CommandOrControl+C');
    if(!flag) {
      // 热键注册
      await register('CommandOrControl+Shift+C', () => {
        console.log('Shortcut triggered');
      });

    }

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

label {
  align-self: flex-start;
}

.input-area {
  display: flex;
}

.input-area input {
  flex: 1;
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
