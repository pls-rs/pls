<script setup lang="ts">
import { computed, onMounted } from "vue";

const props = defineProps({
  color: {
    type: String,
    validator: (val: string) =>
      [
        /^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$/,
        /^rgb\(\s?(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\s?,\s?(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\s?,\s?(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\s?\)$/,
      ].some((item) => item.test(val)),
  },
});
</script>

<template>
  <code>
    <span class="dot" :style="{ backgroundColor: props.color }" />{{ color }}
  </code>
</template>

<style scoped>
.dot {
  display: inline-block;
  width: 0.75em;
  height: 0.75em;
  border-radius: 99em;
  margin-inline-end: 0.25rem;
}
</style>
