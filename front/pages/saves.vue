<template>
  <div class="saves">
    <!-- Link back to search -->
    <Icon href="/">
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
    </Icon>

    <div v-if="!loading">
      <!-- List of businesses -->
      <Businesses
        v-if="businesses !== undefined"
        @pull="businessPull"
        :businesses="businesses"
        :saved="saved"
      />

      <!-- Error message -->
      <p class="error" v-else>{{ error }}</p>
    </div>

    <!-- The loading spinner -->
    <Spinner v-else />
  </div>
</template>

<script lang="ts" setup>
// Set HTML head
useHead({
  title: "Busquery",
  meta: [
    {
      name: "apple-mobile-web-app-capable",
      content: "yes",
    },
    {
      name: "apple-mobile-web-app-status-bar-style",
      content: "black-translucent",
    },
    {
      name: "apple-mobile-web-app-orientations",
      content: "portrait-any",
    },
    {
      name: "theme-color",
      content: "#f7a062",
      media: "(prefers-color-scheme: dark)",
    },
  ],
});
</script>

<script lang="ts">
import axios from "axios";

export default {
  data: () => {
    const data: {
      // DOM
      loading: boolean;
      error: string | undefined;
      // Data
      saved: Array<Saved> | undefined;
      businesses: Array<BrregBusiness> | undefined;
    } = {
      // DOM
      loading: false,
      error: undefined,
      // Data
      saved: undefined,
      businesses: undefined,
    };

    return data;
  },
  async mounted() {
    this.loading = true;

    // Get all saved businesses
    await this.businessPull(false);
    this.loading = false;
  },
  methods: {
    async businessPull(update: boolean): Promise<void> {
      this.saved = await getSavedBusinesses();

      // Construct all business data
      if (this.saved != undefined) {
        this.businesses = await Promise.all(
          this.saved?.map(async (s): Promise<BrregBusiness> => {
            const response = await axios.get<BrregBusiness>(
              `https://data.brreg.no/enhetsregisteret/api/enheter/${s.org}`
            );

            return response.data;
          })
        );

        if (update) {
          this.$forceUpdate();
        }
      }
    },
  },
};
</script>

<style lang="scss">
.saves {
  margin-top: 100px;
}
</style>
