<script setup>
import { ref } from 'vue'
import { emit, listen } from '@tauri-apps/api/event'
import { convertFileSrc } from '@tauri-apps/api/tauri'

defineProps({
  msg: String
})

const count = ref(0)
const url = ref('tmp')

// let unlisten;
// async function setupListener() {
//   unlisten = await listen('display-image', event => {
//     // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
//     // event.payload is the payload object
//     const filePath = event.payload
//     console.log(filePath)
//     console.log(convertFileSrc(filePath))

//     url.value = convertFileSrc(filePath)  
// })
// }
// setupListener();


listen('display-image', event => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  const filePath = event.payload
  console.log(filePath)
  console.log(convertFileSrc(filePath))

  url.value = convertFileSrc(filePath)  
})


</script>

<template>
  <h1>{{ msg }}</h1>

  <el-image style="width: 100px; height: 100px" :src="url" fit="fill" />

  <p>
    Recommended IDE setup:
    <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
    +
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
  </p>

  <p>
    <a href="https://vitejs.dev/guide/features.html" target="_blank">
      Vite Documentation
    </a>
    |
    <a href="https://v3.vuejs.org/" target="_blank">Vue 3 Documentation</a>
  </p>

  <el-button @click="count++">count is: {{ count }}</el-button>
  <p>
    Edit
    <code>components/HelloWorld.vue</code> to test hot module replacement.
  </p>

  
</template>

<style scoped>
a {
  color: #42b983;
}
</style>
