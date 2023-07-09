<template>
  <div class="home">
    <!-- The Busquery logo -->
    <p class="home__logo">Busquery</p>

    <!-- Saved list icon -->
    <svg
      @click="$router.push('/saves')"
      class="saved"
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
        <!-- Entry modal -->
        <dialog class="entries__modal" ref="entriesModal" @click="modalClick">
          <div class="entries__modal__body" v-if="selectedEntry !== undefined">
            <h3 class="entries__modal__body__title">
              {{ selectedEntry?.navn }}
            </h3>
            <p class="entries__modal__body__paragraph">Notater</p>
            <textarea
              class="entries__modal__body__input"
              cols="50"
              rows="10"
              spellcheck="false"
              ref="modalText"
            ></textarea>
            <br />

            <button
              v-if="!entryIsSaved(selectedEntry)"
              class="entries__modal__body__btn entries__modal__body__btn__left"
              @click="modalSave"
            >
              Lagre
            </button>
            <button
              v-else
              class="entries__modal__body__btn entries__modal__body__btn__left"
              @click="modalUpdate"
            >
              Oppdater
            </button>

            <button
              class="entries__modal__body__btn entries__modal__body__btn__right"
              @click="modalDelete"
            >
              Slett
            </button>
          </div>
        </dialog>

        <!-- Business entry -->
        <li class="entry" v-for="entry in (entries as Array<BrregEntry>)">
          <!-- Menu button -->
          <button
            class="entry__btn"
            :class="{ entry__btn__active: entryIsSaved(entry) }"
            @click="entryModal(entry)"
          >
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

          <!-- Business name -->
          <h3>{{ entry.navn }}</h3>
          <!-- Business address -->
          <p v-if="entry.forretningsadresse !== undefined">
            <a
              :href="
                'https://www.google.com/maps/search/?api=1&query=' +
                entry.forretningsadresse.adresse.join(' ') +
                ` ${entry.forretningsadresse.poststed}` +
                ` ${entry.forretningsadresse.postnummer || ''}` +
                ` ${entry.forretningsadresse.land}`
              "
              target="_blank"
              rel="noopener"
            >
              {{ entry.forretningsadresse!.adresse[0] }}
              <svg
                style="height: 15px"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  d="M16.555 5.412a8.028 8.028 0 00-3.503-2.81 14.899 14.899 0 011.663 4.472 8.547 8.547 0 001.84-1.662zM13.326 7.825a13.43 13.43 0 00-2.413-5.773 8.087 8.087 0 00-1.826 0 13.43 13.43 0 00-2.413 5.773A8.473 8.473 0 0010 8.5c1.18 0 2.304-.24 3.326-.675zM6.514 9.376A9.98 9.98 0 0010 10c1.226 0 2.4-.22 3.486-.624a13.54 13.54 0 01-.351 3.759A13.54 13.54 0 0110 13.5c-1.079 0-2.128-.127-3.134-.366a13.538 13.538 0 01-.352-3.758zM5.285 7.074a14.9 14.9 0 011.663-4.471 8.028 8.028 0 00-3.503 2.81c.529.638 1.149 1.199 1.84 1.66zM17.334 6.798a7.973 7.973 0 01.614 4.115 13.47 13.47 0 01-3.178 1.72 15.093 15.093 0 00.174-3.939 10.043 10.043 0 002.39-1.896zM2.666 6.798a10.042 10.042 0 002.39 1.896 15.196 15.196 0 00.174 3.94 13.472 13.472 0 01-3.178-1.72 7.973 7.973 0 01.615-4.115zM10 15c.898 0 1.778-.079 2.633-.23a13.473 13.473 0 01-1.72 3.178 8.099 8.099 0 01-1.826 0 13.47 13.47 0 01-1.72-3.178c.855.151 1.735.23 2.633.23zM14.357 14.357a14.912 14.912 0 01-1.305 3.04 8.027 8.027 0 004.345-4.345c-.953.542-1.971.981-3.04 1.305zM6.948 17.397a8.027 8.027 0 01-4.345-4.345c.953.542 1.971.981 3.04 1.305a14.912 14.912 0 001.305 3.04z"
                />
              </svg>
            </a>
          </p>
          <!-- Business website -->
          <p v-if="entry.hjemmeside !== undefined">
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
          <p v-else><b>Har ingen hjemmeside</b></p>
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

<script lang="ts" setup>
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
  postnummer: number | undefined;
  poststed: string;
  adresse: Array<string>;
  kommune: string;
  kommunenummer: number;
}

interface Saved {
  org: number;
  notes: string;
}

export default {
  data: () => {
    const data: {
      loading: boolean;
      error: string | undefined;
      saved: Array<Saved> | undefined;
      query: string;
      filterPlace: number | undefined;
      entries: Array<BrregEntry> | undefined;
      selectedEntry: BrregEntry | undefined;
    } = {
      loading: false,
      error: undefined,
      saved: undefined,
      query: "",
      filterPlace: undefined,
      entries: undefined,
      selectedEntry: undefined,
    };

    return data;
  },
  async mounted() {
    // Set on scroll handler function
    window.onscroll = this.scrollHandle;

    // Get all saved businesses
    this.pullEntries();
  },
  methods: {
    pullEntries(): void {
      const apiUrl = useAppConfig().apiUrl;

      axios(apiUrl + "business/all")
        .then((r) => (this.saved = r.data))
        .catch((err) => console.error(err));
    },
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
    entryIsSaved(entry: BrregEntry): boolean {
      let comps = this.saved?.map((s) => s.org == entry.organisasjonsnummer);

      return comps?.includes(true) || false;
    },
    entryModal(entry: BrregEntry): void {
      this.selectedEntry = entry;
      (this.$refs["entriesModal"] as HTMLDialogElement).showModal();

      this.$nextTick(() => {
        if (this.entryIsSaved(entry)) {
          let s = this.saved?.find(
            (element) => element.org == entry.organisasjonsnummer
          );
          (this.$refs["modalText"] as HTMLTextAreaElement).value =
            s?.notes || "";
        }
      });
    },
    modalClose(): void {
      (this.$refs["modalText"] as HTMLTextAreaElement).value = "";
      (this.$refs["entriesModal"] as HTMLDialogElement).close();
      this.selectedEntry = undefined;
    },
    modalClick(event: Event): void {
      if (event.target == (this.$refs["entriesModal"] as HTMLDialogElement)) {
        this.modalClose();
      }
    },
    modalSave(): void {
      const apiUrl = useAppConfig().apiUrl;

      // Save business to list
      axios
        .post(
          apiUrl + `business/${this.selectedEntry?.organisasjonsnummer}/add`,
          {
            notes: (this.$refs["modalText"] as HTMLTextAreaElement).value,
          }
        )
        .catch((err) => console.error(err))
        .then(() => {
          // Update entries
          this.modalClose();
          this.pullEntries();
          this.$forceUpdate();
        });
    },
    modalUpdate(): void {
      const apiUrl = useAppConfig().apiUrl;

      // Save business to list
      axios
        .put(
          apiUrl + `business/${this.selectedEntry?.organisasjonsnummer}/update`,
          {
            notes: (this.$refs["modalText"] as HTMLTextAreaElement).value,
          }
        )
        .catch((err) => console.error(err))
        .then(() => {
          // Update entries
          this.modalClose();
          this.pullEntries();
        });
    },
    modalDelete(): void {
      const apiUrl = useAppConfig().apiUrl;

      // Delete business from list
      axios
        .delete(
          apiUrl + `business/${this.selectedEntry?.organisasjonsnummer}/remove`
        )
        .catch((err) => console.error(err))
        .then(() => {
          // Update entries
          this.modalClose();
          this.pullEntries();
          this.$forceUpdate();
        });
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

  $savedPadding: 30px;
  .saved {
    height: 30px;

    position: absolute;
    top: $savedPadding;
    left: $savedPadding;

    color: white;
    cursor: pointer;
  }
  .saved:active {
    color: var(--color);
  }

  .form {
    width: 100%;

    display: flex;
    flex-direction: row;
    border-radius: 0;

    z-index: 100;

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

    &__modal {
      padding: 0;

      border: none;

      &__body {
        width: 100%;
        height: 100%;

        position: relative;

        padding: 10px;
        margin: 0;

        color: var(--color);
        background-color: white;

        &__title {
          margin-top: 0;
        }

        &__paragraph {
          margin: 0;
        }

        &__input {
          width: 100%;

          border: 4px solid var(--color);
          border-radius: 0;
        }
        &__input:focus {
          outline: none;
        }

        &__btn {
          background: var(--color);
          color: white;

          margin: 5px 0 10px 0;
          padding: 5px 10px;

          border: none;
          cursor: pointer;

          font-size: 14px;

          &__left {
            float: left;
          }

          &__right {
            float: right;
          }
        }
        &__btn:active {
          background-color: var(--active);
        }
      }
    }

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
        width: calc(100% - (#{$entryBtnSize} + 10px));
        text-overflow: ellipsis;
        white-space: nowrap;
        overflow: hidden;

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
