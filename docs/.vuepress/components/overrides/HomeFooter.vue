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
  <footer class="footer">
    <p>
      <code>pls</code> is
      <a href="https://pypi.org/project/pls/" target="_blank">free</a> and
      <a href="https://github.com/dhruvkb/pls" target="_blank">open-source</a>
      software. <strong>{{ approximateStars }}+ stars</strong> on GitHub.
    </p>
    <p>
      You can help <a href="https://github.com/sponsors/dhruvkb">sponsor</a>
      its development.
    </p>
  </footer>
</template>

<style scoped lang="scss">
footer.footer {
  p {
    margin-bottom: 0;
    margin-top: 0.5em;

    &:first-of-type {
      margin-top: 0;
    }
  }
}
</style>
