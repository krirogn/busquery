// Format ISO date to dd/mm/yyyy
export const dateFormat = (rawDate: string): string => {
  return new Date(rawDate).toJSON().slice(0, 10).split("-").reverse().join("/");
};
