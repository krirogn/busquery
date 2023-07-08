<template>
  <div class="home">
    <!-- The Busquery logo -->
    <p class="home__logo">Busquery</p>

    <!-- The search bar -->
    <div class="form" ref="form">
      <!-- The text field -->
      <input
        class="form__search"
        type="text"
        placeholder="Søk bedrifter..."
        v-model="query"
        v-on:keyup.enter="search"
        ref="search"
        autofocus
      />

      <!-- The municipality drop-down -->
      <select class="form__select" v-model="filterPlace">
        <option :value="undefined">Alle</option>
        <option
          v-for="municipality in municipalities?.body"
          :value="municipality['kommunenummer']"
        >
          {{ municipality["kommunenavnNorsk"] }}
        </option>
      </select>

      <!-- The search icon -->
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
      <!-- The list of businesses -->
      <ul class="entries" ref="entries" v-if="entries !== undefined">
        <!-- Business entry -->
        <li class="entry" v-for="entry in (entries as Array<BrregEntry>)">
          <!-- Menu button -->
          <button class="entry__btn">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-6 h-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
              />
            </svg>
          </button>

          <!-- Entry modal -->
          <dialog></dialog>

          <!-- Business name -->
          <h3>{{ entry.navn }}</h3>
          <!-- Business address -->
          <p v-if="entry.forretningsadresse !== undefined">
            {{ entry.forretningsadresse!.kommune }}
          </p>
          <!-- Business website -->
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
          <!-- Number of employees -->
          <p>Ansatte: {{ entry.antallAnsatte }}</p>
          <!-- Business established date -->
          <p v-if="entry.stiftelsesdato">
            Stiftelse: {{ dateFormat(entry.stiftelsesdato!) }}
          </p>
          <!-- Business description -->
          <p v-if="entry.naeringskode1 !== undefined">
            Næring: {{ entry.naeringskode1!.beskrivelse }}
          </p>
        </li>
      </ul>

      <!-- Error message -->
      <p class="error" v-else>{{ error }}</p>
    </div>
    <!-- The loading spinner -->
    <div class="loader" v-else></div>
  </div>
</template>

<script lang="ts">
import axios from "axios";
import { toRefs, toRef } from "vue";

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
  async setup() {
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

    const { data: municipalities } = await useAsyncData("municipalities", () =>
      queryContent("/municipalities").findOne()
    );

    return {
      municipalities,
    };
  },
  data: () => {
    const data: {
      loading: boolean;
      error: string | undefined;
      saved: Array<Object> | undefined;
      query: string;
      filterPlace: number | undefined;
      entries: Array<BrregEntry> | undefined;
    } = {
      loading: false,
      error: undefined,
      saved: undefined,
      query: "",
      filterPlace: undefined,
      entries: undefined,
    };

    return data;
  },
  async mounted() {
    window.onscroll = this.scrollHandle;

    // Get all saved businesses
    const apiUrl = useAppConfig().apiUrl;

    axios(apiUrl + "business/all")
      .then((r) => (this.saved = r.data))
      .catch((err) => console.error(err));
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
      position: relative;

      margin-bottom: 20px;
      padding: 10px;

      color: var(--color);
      background-color: white;

      $entryBtnSize: 25px;
      $entryBtnPadding: 10px;
      &__btn {
        width: $entryBtnSize;
        height: $entryBtnSize;

        position: absolute;
        top: $entryBtnPadding;
        right: $entryBtnPadding;

        border-radius: 0;
        border: 2px solid var(--color);

        color: var(--color);
        background-color: white;
        cursor: pointer;

        $entryBtnIconSize: 18px;
        svg {
          width: $entryBtnIconSize;
          height: $entryBtnIconSize;

          position: absolute;
          top: 50%;
          left: 50%;
          transform: translate(-50%, -50%);
        }
      }
      &__btn:active {
        border: none;

        color: white;
        background-color: var(--color);
      }

      &__btn__active {
        border: none;

        color: white;
        background-color: var(--color);
      }
      &__btn__active:active {
        background-color: var(--active);
      }

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
