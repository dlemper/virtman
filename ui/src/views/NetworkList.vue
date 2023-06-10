<script setup>
import { ref } from "vue";
import { mande } from "mande";

const networkApi = mande("/api/network");
const networks = ref();
networkApi.get().then(i => { networks.value = i });

const interfaceApi = mande("/api/interface");
const interfaces = ref();
interfaceApi.get().then(i => { interfaces.value = i });

function formatProtocol(protos) {
  return protos.filter(proto => proto.ip).map(proto => `${proto["@family"]}: ${proto.ip["@address"]}/${proto.ip["@prefix"]}`).join(", ")
}
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
        <tr v-for="(data, idx) in interfaces" :key="idx">
          <td>{{ data.interface["@type"] }}</td>
          <td>{{ data.interface["@name"] }}</td>
          <td>{{ formatProtocol(data.interface.protocol) }}</td>
          <td>{{ data.interface.bridge }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>