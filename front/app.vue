<template>
  <div class="container">
    <NuxtPage />
  </div>
</template>

<script lang="ts" setup>
console.info(
  `%c888888b.                                       d8b
888  "88b                                      Y8P
888  .88P
8888888K.  888  888 .d8888b   .d88888 888  888 888 888d888 888  888
888  "Y88b 888  888 88K      d88" 888 888  888 888 888P"   888  888
888    888 888  888 "Y8888b. 888  888 888  888 888 888     888  888
888   d88P Y88b 888      X88 Y88b 888 Y88b 888 888 888     Y88b 888
8888888P"   "Y88888  88888P'  "Y88888  "Y88888 888 888      "Y88888
                                  888                           888
                                  888                      Y8b d88P
                                  888                       "Y88P"`,
  "color: #f7a062"
);
</script>

<style lang="scss">
// Imports
// Montserrat-Alt1 from https://github.com/Differentunic/Montserrat-Alt1
@import url("/fonts/Montserrat-Alt1/Montserrat-Alt1.css");

// Variables
:root {
  // Orange
  --color: #f7a062;
  --accent: #ffe5d2;
  --active: #fc954b;

  // Blue
  // --color: #62bef7;
  // --accent: #d2f5ff;
  // --active: #4ba9fc;
}

// Init
html,
body {
  min-height: 100vh;
  overflow-x: hidden;

  margin: 0;
  padding: 0;

  background: linear-gradient(to right top, var(--accent) 30%, var(--color));
}

* {
  box-sizing: border-box;
}

a {
  text-decoration: underline solid 2px;
  color: var(--color);
}

// DOM
.container {
  max-width: 800px;
  padding: 0 20px;
  margin-left: auto;
  margin-right: auto;

  font-size: 18px;
  font-family: "Poppins", sans-serif;
  color: black;
}
</style>
