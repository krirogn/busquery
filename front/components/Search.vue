<template>
  <div class="search" :class="{ 'sticky-search': sticky }">
    <!-- The text field -->
    <input
      class="search__field"
      type="text"
      placeholder="Søk bedrifter..."
      v-model="query"
      v-on:keyup.enter="search"
      ref="search"
      autofocus
    />

    <!-- The municipality drop-down -->
    <select class="search__select" v-model="filterPlace">
      <option :value="undefined">Alle</option>
      <option
        v-for="municipality in municipalities?.body"
        :value="municipality['kommunenummer']"
      >
        {{ municipality["kommunenavnNorsk"] }}
      </option>
    </select>

    <!-- The search icon -->
    <button class="search__btn" @click="search">
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
</template>

<script lang="ts" setup>
// Props
defineProps({
  sticky: {
    type: Boolean,
    required: false,
    default: false,
  },
});

// Get the municipalities.json from content
const { data: municipalities } = await useAsyncData("municipalities", () =>
  queryContent("/municipalities").findOne()
);
</script>

<script lang="ts">
import axios from "axios";

export default {
  data: () => {
    const data: {
      // Search
      query: string;
      filterPlace: number | undefined;
    } = {
      // Search
      query: "",
      filterPlace: undefined,
    };

    return data;
  },
  emits: ["load", "error", "businesses"],
  methods: {
    search(): void {
      // Remove focus on input
      (this.$refs["search"] as HTMLInputElement).blur();

      // Get data from Brønnøysundregisteret
      this.$emit("load", true);
      let request = `https://data.brreg.no/enhetsregisteret/api/enheter?navn=${this.query}&size=60`;

      // Add location filter if chosen
      if (this.filterPlace !== undefined) {
        request += `&kommunenummer=${this.filterPlace}`;
      }

      // Run request
      axios
        .get<BrregSearch>(request)
        .then((req) => {
          this.$emit("load", false);

          if (req.data._embedded === undefined) {
            this.$emit("businesses", undefined);
            this.$emit("error", "Ingen resultat");
          } else {
            this.$emit("businesses", req.data._embedded.enheter);
            this.$emit("error", undefined);
          }
        })
        .catch((err) => {
          this.$emit("load", false);
          console.error(err);
        });
    },
  },
};
</script>

<style lang="scss">
.search {
  width: 100%;

  display: flex;
  flex-direction: row;
  border-radius: 0;

  z-index: 100;

  &__field {
    width: 100%;

    flex-grow: 2;

    padding: 10px;
    border: 10px solid var(--color);
    border-radius: 0;
    border-right: none;

    font-size: 20px;
  }
  &__field:focus {
    outline: none;
  }

  &__select {
    background: var(--color);
    color: white;

    border: none;
    border-radius: 0;
    cursor: pointer;
    padding-left: 10px;
  }
  &__select:active {
    background: var(--active);
  }

  $iconSize: 35px;
  &__btn {
    background: var(--color);
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
    background: var(--active);
  }
}

.sticky-search {
  max-width: calc(800px - (20px * 2));
  width: calc(100% - 40px) !important;

  position: fixed;
  margin-top: -340px;
}
</style>
