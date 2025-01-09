<script setup lang="ts">
import { ref, reactive } from 'vue'
//import HelloWorld from './components/HelloWorld.vue'

// These are placeholder values, replace with actual data fetching and handling.
const shellInput = ref('')
const shellHistory = reactive([])
const graphVisible = ref(true)
const tableVisible = ref(true)
const tableData = reactive([])

function runShellCommand() {
  sendCommand(shellInput.value)
}

function navigateToLine(line) {
  // TODO: Implement the logic to navigate to a specific line in table and render the associated graph.
}

async function sendCommand(command: string): Promise<void> {
    const response = await fetch('/bex/shell', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({ command }) });

    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    } else {
        const jsonResponse = await response.json();
        console.log(jsonResponse);
    }
}
</script>

<template>
  <div id="app">
    <h1>visibex</h1>
    <div class="left-panel">
      <div class="history">
        <div v-for="item in shellHistory" :key="item.id">
          {{ item.cmd }}
          {{ item.stack }}
          {{ item.out }}
        </div>
      </div>
      <input type="text" v-model="shellInput" @keyup.enter="runShellCommand"/>
      <button @click="runShellCommand">Run</button>
    </div>
    <div class="right-panel">
      <div v-show="graphVisible">
        <h3>Graph</h3>
        <!-- Insert Graph component here -->
      </div>
      <div v-show="tableVisible">
        <h3>Table</h3>
        <table>
          <tr v-for="row in tableData" :key="row.id">
            <td>{{ row.id }}</td>
            <td>{{ row.cost }}</td>
            <td>{{ row.op }}</td>
            <td v-for="arg in row.args" :key="arg">
              <a @click="navigateToLine(arg)">{{ arg }}</a>
            </td>
          </tr>
        </table>
      </div>
    </div>
  </div>
</template>

<style scoped>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: flex-start;
  margin-top: 60px;
}
</style>
