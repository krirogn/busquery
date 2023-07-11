<template>
  <div
    class="icon"
    :style="{
      width: size,
      height: size,
      top: padding,
      right: padding,
    }"
    @click="$router.push(href)"
  >
    <slot />
  </div>
</template>

<script lang="ts" setup>
defineProps({
  href: {
    type: String,
    required: true,
  },
  size: {
    type: String,
    required: false,
    default: "30px",
  },
  padding: {
    type: String,
    required: false,
    default: "30px",
  },
});
</script>

<style lang="scss">
.icon {
  position: absolute;

  color: white;
  cursor: pointer;

  svg {
    width: 100%;
    height: 100%;
  }
}
.icon:active {
  color: var(--color);
}
</style>
