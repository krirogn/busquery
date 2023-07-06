<template>
  <div class="home">
    <p class="home__logo">Busquery</p>

    <div class="form" ref="form">
      <input
        class="form__search"
        type="text"
        placeholder="Søk bedrifter..."
        v-model="query"
        v-on:keyup.enter="search"
        ref="search"
        autofocus
      />
      <select class="form__select" v-model="filterPlace">
        <option :value="undefined">Alle</option>
        <option
          v-for="municipality in municipalities?.body"
          :value="municipality['kommunenummer']"
        >
          {{ municipality["kommunenavnNorsk"] }}
        </option>
      </select>
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

    <div v-if="!loading">
      <ul class="entries" ref="entries" v-if="entries !== undefined">
        <li class="entry" v-for="entry in (entries as Array<BrregEntry>)">
          <h3>{{ entry.navn }}</h3>
          <p v-if="entry.forretningsadresse !== undefined">
            {{ entry.forretningsadresse!.kommune }}
          </p>
          <p>
            <a
              :href="
                entry.hjemmeside?.indexOf('://') === -1
                  ? 'http://' + entry.hjemmeside
                  : entry.hjemmeside
              "
              target="_blank"
              rel="noopener"
              >{{ entry.hjemmeside }}</a
            >
          </p>
          <p>Ansatte: {{ entry.antallAnsatte }}</p>
          <p v-if="entry.stiftelsesdato">
            Stiftelse: {{ dateFormat(entry.stiftelsesdato!) }}
          </p>
          <p v-if="entry.naeringskode1 !== undefined">
            Næring: {{ entry.naeringskode1!.beskrivelse }}
          </p>
        </li>
      </ul>
      <p class="error" v-else>{{ error }}</p>
    </div>
    <div class="loader" v-else></div>
  </div>
</template>

<script lang="ts" setup>
const { data: municipalities } = await useAsyncData("municipalities", () =>
  queryContent("/municipalities").findOne()
);

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

interface BrregSearch {
  _embedded:
    | {
        enheter: Array<BrregEntry>;
      }
    | undefined;
}

interface BrregEntry {
  organisasjonsnummer: number;
  navn: string;
  organisasjonsform: Object;
  hjemmeside: string | undefined;
  registreringsdatoEnhetsregisteret: Date | undefined;
  frivilligMvaRegistrertBeskrivelser: Array<string> | undefined;
  naeringskode1:
    | {
        beskrivelse: string;
        kode: number;
      }
    | undefined;
  antallAnsatte: number;
  forretningsadresse: BrregAdresse | undefined;
  stiftelsesdato: string | undefined;
  sisteInnsendteAarsregnskap: Date;
  konkurs: boolean;
  underAvvikling: boolean;
  underTvangsavviklingEllerTvangsopplosning: boolean;
}

interface BrregAdresse {
  land: string;
  landkode: string;
  postnummer: number;
  poststed: string;
  adresse: Array<string>;
  kommune: string;
  kommunenummer: number;
}

export default {
  data: () => {
    const data: {
      loading: boolean;
      error: string | undefined;
      query: string;
      filterPlace: number | undefined;
      entries: Array<BrregEntry> | undefined;
    } = {
      loading: false,
      error: undefined,
      query: "",
      filterPlace: undefined,
      entries: undefined,
    };

    return data;
  },
  mounted() {
    window.onscroll = this.scrollHandle;
  },
  methods: {
    search(): void {
      // Remove focus on input
      (this.$refs["search"] as HTMLInputElement).blur();

      // Get data from Brønnøysundregisteret
      this.loading = true;
      let request = `https://data.brreg.no/enhetsregisteret/api/enheter?navn=${this.query}&size=60`;

      if (this.filterPlace !== undefined) {
        request += `&kommunenummer=${this.filterPlace}`;
      }

      axios
        .get<BrregSearch>(request)
        .then((req) => {
          this.loading = false;

          if (req.data._embedded === undefined) {
            this.entries = undefined;
            this.error = "Ingen resultat";
          } else {
            this.error = undefined;
            this.entries = req.data._embedded.enheter;
          }
        })
        .catch((err) => {
          this.loading = false;

          console.error(err);
        });
    },
    scrollHandle(_event: Event): void {
      if (window.scrollY > 340) {
        (this.$refs["form"] as HTMLDivElement).classList.add("sticky-search");
        (this.$refs["entries"] as HTMLDivElement).classList.add("shift-down");
      } else {
        (this.$refs["form"] as HTMLDivElement).classList.remove(
          "sticky-search"
        );
        (this.$refs["entries"] as HTMLDivElement).classList.remove(
          "shift-down"
        );
      }
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

    user-select: none;
  }

  .form {
    width: 100%;

    display: flex;
    flex-direction: row;
    border-radius: 0;

    &__search {
      width: 100%;

      flex-grow: 2;

      padding: 10px;
      border: 10px solid var(--color);
      border-radius: 0;
      border-right: none;

      font-size: 20px;
    }
    &__search:focus {
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

  .entries {
    margin: 20px 10px;
    padding: 0;

    list-style: none;

    .entry {
      margin-bottom: 20px;
      padding: 10px;

      color: var(--color);
      background-color: white;

      p {
        margin: 5px 0;
      }

      h3 {
        margin-top: 0;
      }
    }
  }

  .error {
    width: 100%;
    text-align: center;

    margin-top: 30px;
    font-size: 21px;
  }

  $loaderSize: 60px;
  $loaderThickness: 8px;
  .loader {
    width: $loaderSize;
    height: $loaderSize;

    margin-top: 30px;
    margin-right: auto;
    margin-left: auto;

    border: $loaderThickness solid white;
    border-top: $loaderThickness solid var(--color);
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
}

.sticky-search {
  max-width: calc(800px - (20px * 2));
  width: calc(100% - 40px) !important;

  position: fixed;
  margin-top: -340px;
}

.shift-down {
  margin-top: 155px !important;
}
</style>
