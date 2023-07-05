<template>
  <div class="home">
    <p class="home__logo">Busquery</p>

    <div class="form">
      <input
        class="form__search"
        type="text"
        placeholder="Søk bedrifter..."
        v-model="query"
        v-on:keyup.enter="search"
        ref="search"
        autofocus
      />
      <button class="form__btn" @click="search">
        <!-- magnifying-glass from https://heroicons.com/ -->
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
          />
        </svg>
      </button>
    </div>

    <p>{{ query }}</p>
  </div>
</template>

<script lang="ts">
import axios from "axios";

export default {
  data: () => {
    const data: {
      query: string;
      businesses: undefined;
    } = {
      query: "",
      businesses: undefined,
    };

    return data;
  },
  methods: {
    search(): void {
      (this.$refs['search'] as HTMLInputElement).blur();
      alert(`Searching for ${this.query}`);
    },
  },
};
</script>

<style lang="scss">
.home {
  margin-top: 200px;

  &__logo {
    width: 100%;

    color: white;

    text-align: center;
    font-family: "Montserrat-Alt1";
    font-size: 4em;
    font-weight: 700;
  }

  .form {
    width: 100%;

    display: flex;
    flex-direction: row;

    &__search {
      width: 100%;

      flex-grow: 2;

      padding: 10px;
      border: 10px solid #f7a062;
      border-right: none;

      font-size: 20px;
    }
    &__search:focus {
      outline: none;
    }

    $iconSize: 35px;
    &__btn {
      background: #f7a062;
      color: white;

      border: none;
      cursor: pointer;
      padding: 0 15px;

      svg {
        height: $iconSize;
        width: $iconSize;
      }
    }
    &__btn:active {
      background: #fc954b;
    }
  }
}
</style>
