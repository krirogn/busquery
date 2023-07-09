export { BrregSearch, BrregBusiness, BrregAdresse, Saved };

declare global {
  interface BrregSearch {
    _embedded:
      | {
          enheter: Array<BrregBusiness>;
        }
      | undefined;
  }

  interface BrregBusiness {
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
}
