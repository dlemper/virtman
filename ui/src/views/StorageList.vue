<script setup>
import { ref } from "vue";
import { mande } from "mande";

const storageApi = mande("/api/storage");
const images = ref();

storageApi.get().then((i) => {
  images.value = i;
});

// from https://stackoverflow.com/a/20459666
function fileSize(size) {
  var i = Math.floor(Math.log(size) / Math.log(1024));
  return (
    (size / Math.pow(1024, i)).toFixed(2) * 1 +
    " " +
    ["B", "kB", "MB", "GB", "TB"][i]
  );
}
</script>

<template>
  <div class="overflow-x-auto">
    <table class="table table-zebra w-full">
      <!-- head -->
      <thead>
        <tr>
          <th>Name</th>
          <th>Path</th>
          <th>Type</th>
          <th>Capacity</th>
          <th>Allocation</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(image, idx) in images" :key="idx">
          <td>{{ image.name }}</td>
          <td>{{ image.path }}</td>
          <td>{{ image.kind }}</td>
          <td>{{ fileSize(image.capacity) }}</td>
          <td>{{ fileSize(image.allocation) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
