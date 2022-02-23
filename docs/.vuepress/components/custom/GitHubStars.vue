<script setup lang="ts">
import { computed, onMounted, ref } from "vue";

const stars = ref(0);
onMounted(async () => {
  const owner = "dhruvkb";
  const repo = "pls";
  const data = await (
    await fetch(`https://api.github.com/repos/${owner}/${repo}`)
  ).json();
  stars.value = data.stargazers_count;
});

const approximateStars = computed(() => Math.trunc(stars.value / 10) * 10);
</script>

<template>
  <strong>{{ approximateStars }}+ stars</strong>
</template>
