<script setup>
import { ref } from "vue";
import { mande } from "mande";
import PauseIcon from "../components/icons/PauseIcon.vue";
import PlayIcon from "../components/icons/PlayIcon.vue";
//import RFB from "@novnc/novnc/core/rfb.js";
const vmApi = mande("/api/vm");

const vmList = ref();
vmApi.get().then((v) => {
  vmList.value = v;
});

function resume(name) {
  vmApi.patch(`${name}/resume`);
}

function suspend(name) {
  vmApi.patch(`${name}/suspend`);
}

function start(name) {
  vmApi.patch(`${name}/start`);
}
</script>

<template>
  <div class="overflow-x-auto">
    <table class="table w-full">
      <thead>
        <tr>
          <th scope="col">ID</th>
          <th scope="col">Name</th>
          <th scope="col">Active</th>
          <th scope="col"></th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="(vmItem, idx) in vmList" :key="idx">
          <th scope="row">{{ vmItem.id }}</th>
          <td>{{ vmItem.name }}</td>
          <td>{{ vmItem.active }}</td>
          <td>
            <div class="btn-group">
              <button class="btn btn-sm" @click="resume(vmItem.name)">
                <play-icon></play-icon>
              </button>
              <button class="btn btn-sm" @click="suspend(vmItem.name)">
                <pause-icon></pause-icon>
              </button>
              <button class="btn btn-sm" @click="start(vmItem.name)">
                <play-icon></play-icon>
              </button>
            </div>
          </td>
        </tr>
      </tbody>

      <!--<tfoot>
        <tr>
          <th scope="col">#</th>
          <td scope="col">Total</td>
          <td scope="col">Total</td>
        </tr>
      </tfoot>-->
    </table>
  </div>
</template>