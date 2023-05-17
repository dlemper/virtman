<script setup>
import { ref } from "vue";
import { mande } from "mande";

const networkApi = mande("/api/network");
const networks = ref();
networkApi.get().then(i => { networks.value = i });

const interfaceApi = mande("/api/interface");
const interfaces = ref();
interfaceApi.get().then(i => { interfaces.value = i });
</script>

<template>
  <div class="overflow-x-auto">
    <table class="table table-zebra w-full">
      <!-- head -->
      <thead>
        <tr>
          <th>Name</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(network, idx) in networks" :key="idx">
          <td>{{ network }}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <div class="overflow-x-auto">
    <table class="table table-zebra w-full">
      <!-- head -->
      <thead>
        <tr>
          <th>Type</th>
          <th>Name</th>
          <th>Protocol</th>
          <th>Bridge</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(iface, idx) in interfaces" :key="idx">
          <td>{{ iface["@type"] }}</td>
          <td>{{ iface["@name"] }}</td>
          <td>{{ iface.protocol }}</td>
          <td>{{ iface.bridge }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>