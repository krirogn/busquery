<template>
  <div
    class="spinner"
    :style="{
      width: size,
      height: size,
      border: `${thickness} solid white`,
      borderTop: `${thickness} solid var(--color)`,
    }"
  ></div>
</template>

<script lang="ts" setup>
defineProps({
  size: {
    type: String,
    required: false,
    default: "60px",
  },
  thickness: {
    type: String,
    required: false,
    default: "8px",
  },
});
</script>

<style lang="scss">
.spinner {
  margin-top: 30px;
  margin-right: auto;
  margin-left: auto;

  border-radius: 50%;
  animation: spin 2s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
