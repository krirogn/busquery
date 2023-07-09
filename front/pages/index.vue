<template>
  <div class="home">
    <!-- The Busquery logo -->
    <p class="logo">Busquery</p>

    <!-- Saved list link -->
    <Icon href="/saves">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path
          fill-rule="evenodd"
          d="M6.32 2.577a49.255 49.255 0 0111.36 0c1.497.174 2.57 1.46 2.57 2.93V21a.75.75 0 01-1.085.67L12 18.089l-7.165 3.583A.75.75 0 013.75 21V5.507c0-1.47 1.073-2.756 2.57-2.93z"
          clip-rule="evenodd"
        />
      </svg>
    </Icon>

    <!-- The search bar -->
    <Search
      :sticky="searchSticky"
      @load="searchLoad"
      @error="searchError"
      @businesses="searchBusinesses"
    />

    <div v-if="!loading">
      <Businesses
        v-if="businesses !== undefined"
        @pull="businessPull"
        :businesses="businesses"
        :saved="saved"
        :sticky="businessSticky"
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
export default {
  data: () => {
    const data: {
      // DOM
      loading: boolean;
      error: string | undefined;
      businessSticky: boolean;
      searchSticky: boolean;
      // Data
      saved: Array<Saved> | undefined;
      businesses: Array<BrregBusiness> | undefined;
    } = {
      // DOM
      loading: false,
      error: undefined,
      businessSticky: false,
      searchSticky: false,
      // Data
      saved: undefined,
      businesses: undefined,
    };

    return data;
  },
  beforeUnmount() {
    window.removeEventListener("scroll", this.scrollHandle);
  },
  async mounted() {
    // Set on scroll handler function
    window.addEventListener("scroll", this.scrollHandle);

    // Get all saved businesses
    this.saved = await getSavedBusinesses();
  },
  methods: {
    // Handler
    scrollHandle(): void {
      // If window has scrolled to search bar
      if (window.scrollY > 340) {
        // Make the search sticky
        this.searchSticky = true;
        // Give the business list a sticky offset
        this.businessSticky = true;
      } else {
        // Stop the search from being sticky
        this.searchSticky = false;
        // Remove the sticky offset from the business list
        this.businessSticky = false;
      }
    },
    // Search bar
    searchLoad(loading: boolean): void {
      this.loading = loading;
    },
    searchError(error: string | undefined): void {
      this.error = error;
    },
    searchBusinesses(businesses: Array<BrregBusiness> | undefined): void {
      this.businesses = businesses;
    },
    // Business list
    async businessPull(update: boolean): Promise<void> {
      this.saved = await getSavedBusinesses();

      if (update) {
        this.$forceUpdate();
      }
    },
  },
};
</script>

<style lang="scss">
.home {
  margin-top: 200px;

  .logo {
    width: 100%;

    color: white;

    text-align: center;
    font-family: "Montserrat-Alt1";
    font-size: 4em;
    font-weight: 700;

    user-select: none;
  }

  .error {
    width: 100%;
    text-align: center;

    margin-top: 30px;
    font-size: 21px;
  }
}
</style>
