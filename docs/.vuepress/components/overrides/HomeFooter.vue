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
  <div class="footer">
    <code>pls</code> is
    <a href="https://pypi.org/project/pls/" target="_blank">free</a> and
    <a href="https://github.com/dhruvkb/pls" target="_blank">open-source</a>
    software. <strong>{{ approximateStars }}+ stars</strong> on GitHub.
  </div>
</template>
