import axios from "axios";

export const getSavedBusinesses = async (): Promise<Array<Saved>> => {
  return new Promise((ret, rej) => {
    const apiUrl = useAppConfig().apiUrl;

    axios
      .get<Array<Saved>>(apiUrl + "business/all")
      .then((r) => ret(r.data))
      .catch((err) => {
        console.error(err);
        rej(err);
      });
  });
};
