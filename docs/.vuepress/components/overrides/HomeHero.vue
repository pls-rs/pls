<script setup lang="ts">
import { computed } from "vue";
import { DefaultThemeHomePageFrontmatter } from "@vuepress/theme-default/lib/shared";
import { isArray } from "@vuepress/shared";
import { usePageFrontmatter } from "@vuepress/client";

import AutoLink from "@theme/AutoLink.vue";

const frontmatter = usePageFrontmatter<DefaultThemeHomePageFrontmatter>();

const actions = computed(() => {
  if (!isArray(frontmatter.value.actions)) {
    return [];
  }

  return frontmatter.value.actions.map(({ text, link, type = "primary" }) => ({
    text,
    link,
    type,
  }));
});
</script>

<template>
  <header class="hero">
    <div style="display: flex; justify-content: center; text-align: left">
      <pre style="line-height: 1.2; color: var(--c-brand)">
       ___
      /\_ \
 _____\//\ \     ____
/\ '__`\\ \ \   /',__\
\ \ \L\ \\_\ \_/\__, `\
 \ \ ,__//\____\/\____/
  \ \ \/ \/____/\/___/
   \ \_\
    \/_/
</pre
      >
    </div>

    <p class="description">
      <code style="color: var(--c-brand)">pls</code> is a prettier and powerful
      <code>ls</code> for the pros.
    </p>

    <p v-if="actions.length" class="actions">
      <AutoLink
        v-for="action in actions"
        :key="action.text"
        class="action-button"
        :class="[action.type]"
        :item="action"
      />
    </p>
  </header>
</template>
