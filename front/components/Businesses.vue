<template>
  <div>
    <!-- Business modal -->
    <dialog class="modal" ref="modal" @click="modalClick">
      <div class="modal__body" v-if="selectedBusiness !== undefined">
        <h3 class="modal__body__title">
          {{ selectedBusiness?.navn }}
        </h3>
        <p class="modal__body__paragraph">Notater</p>
        <textarea
          class="modal__body__input"
          cols="50"
          rows="10"
          spellcheck="false"
          ref="modalText"
        ></textarea>
        <br />

        <button
          v-if="!isBusinessSaved(selectedBusiness)"
          class="modal__body__btn modal__body__btn__left"
          @click="modalSave"
        >
          Lagre
        </button>
        <button
          v-else
          class="modal__body__btn modal__body__btn__left"
          @click="modalUpdate"
        >
          Oppdater
        </button>

        <button
          class="modal__body__btn modal__body__btn__right"
          @click="modalDelete"
        >
          Slett
        </button>
      </div>
    </dialog>

    <!-- The list of businesses -->
    <ul
      class="businesses"
      v-if="businesses !== undefined"
      :class="{ 'shift-down': sticky }"
    >
      <!-- Business -->
      <li class="business" v-for="business in businesses">
        <!-- Menu button -->
        <button
          class="business__btn"
          :class="{ business__btn__active: isBusinessSaved(business) }"
          @click="modalOpen(business)"
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
        <h3>{{ business.navn }}</h3>
        <!-- Business address -->
        <p v-if="business.forretningsadresse !== undefined">
          <a
            :href="
              'https://www.google.com/maps/search/?api=1&query=' +
              business.forretningsadresse.adresse.join(' ') +
              ` ${business.forretningsadresse.poststed}` +
              ` ${business.forretningsadresse.postnummer || ''}` +
              ` ${business.forretningsadresse.land}`
            "
            target="_blank"
            rel="noopener"
          >
            {{ business.forretningsadresse!.adresse[0] }}
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
        <p v-if="business.hjemmeside !== undefined">
          <a
            :href="
              business.hjemmeside?.indexOf('://') === -1
                ? 'http://' + business.hjemmeside
                : business.hjemmeside
            "
            target="_blank"
            rel="noopener"
            >{{ business.hjemmeside }}</a
          >
        </p>
        <p v-else><b>Har ingen hjemmeside</b></p>
        <!-- Number of employees -->
        <p>Ansatte: {{ business.antallAnsatte }}</p>
        <!-- Business established date -->
        <p v-if="business.stiftelsesdato">
          Stiftelse: {{ dateFormat(business.stiftelsesdato!) }}
        </p>
        <!-- Business description -->
        <p v-if="business.naeringskode1 !== undefined">
          Næring: {{ business.naeringskode1!.beskrivelse }}
        </p>
      </li>
    </ul>
  </div>
</template>

<script lang="ts" setup>
// Props
defineProps({
  businesses: {
    type: Object as () => Array<BrregBusiness> | undefined,
    required: true,
  },
  saved: {
    type: Object as () => Array<Saved> | undefined,
    required: true,
  },
  sticky: {
    type: Boolean,
    required: false,
    default: false,
  },
});
</script>

<script lang="ts">
import axios from "axios";

export default {
  data: () => {
    const data: {
      selectedBusiness: BrregBusiness | undefined;
    } = {
      selectedBusiness: undefined,
    };

    return data;
  },
  emits: ["pull"],
  methods: {
    isBusinessSaved(business: BrregBusiness): boolean {
      let comps = this.saved?.map((s) => s.org == business.organisasjonsnummer);

      return comps?.includes(true) || false;
    },
    modalOpen(business: BrregBusiness): void {
      this.selectedBusiness = business;
      (this.$refs["modal"] as HTMLDialogElement).showModal();

      this.$nextTick(() => {
        let s = this.saved?.find(
          (element) => element.org == business.organisasjonsnummer
        );
        (this.$refs["modalText"] as HTMLTextAreaElement).value = s?.notes || "";
      });
    },
    modalClose(): void {
      (this.$refs["modalText"] as HTMLTextAreaElement).value = "";
      (this.$refs["modal"] as HTMLDialogElement).close();
      this.selectedBusiness = undefined;
    },
    modalClick(event: Event): void {
      if (event.target == (this.$refs["modal"] as HTMLDialogElement)) {
        this.modalClose();
      }
    },
    modalSave(): void {
      const apiUrl = useAppConfig().apiUrl;

      // Save business to list
      axios
        .post(
          apiUrl + `business/${this.selectedBusiness?.organisasjonsnummer}/add`,
          {
            notes: (this.$refs["modalText"] as HTMLTextAreaElement).value,
          }
        )
        .catch((err) => console.error(err))
        .then(() => {
          // Update business list
          this.modalClose();
          this.$emit("pull", true);
        });
    },
    modalUpdate(): void {
      const apiUrl = useAppConfig().apiUrl;

      // Save business to list
      axios
        .put(
          apiUrl +
            `business/${this.selectedBusiness?.organisasjonsnummer}/update`,
          {
            notes: (this.$refs["modalText"] as HTMLTextAreaElement).value,
          }
        )
        .catch((err) => console.error(err))
        .then(() => {
          // Update business list
          this.modalClose();
          this.$emit("pull", false);
        });
    },
    modalDelete(): void {
      const apiUrl = useAppConfig().apiUrl;

      // Delete business from list
      axios
        .delete(
          apiUrl +
            `business/${this.selectedBusiness?.organisasjonsnummer}/remove`
        )
        .catch((err) => console.error(err))
        .then(() => {
          // Update business list
          this.modalClose();
          this.$emit("pull", true);
        });
    },
  },
};
</script>

<style lang="scss">
.modal {
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

.businesses {
  margin: 20px 10px;
  padding: 0;

  list-style: none;

  .business {
    position: relative;

    margin-bottom: 20px;
    padding: 10px;

    color: var(--color);
    background-color: white;

    $businessBtnSize: 25px;
    $businessBtnPadding: 10px;
    &__btn {
      width: $businessBtnSize;
      height: $businessBtnSize;

      position: absolute;
      top: $businessBtnPadding;
      right: $businessBtnPadding;

      border-radius: 0;
      border: 2px solid var(--color);

      color: var(--color);
      background-color: white;
      cursor: pointer;

      $businessBtnIconSize: 18px;
      svg {
        width: $businessBtnIconSize;
        height: $businessBtnIconSize;

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
      width: calc(100% - (#{$businessBtnSize} + 10px));
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;

      margin-top: 0;
    }
  }
}

.shift-down {
  margin-top: 155px !important;
}
</style>
