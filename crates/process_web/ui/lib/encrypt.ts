import { encode, decode } from "js-base64";

export function encrypt(text: string) {
  const tt = "DpSALt";
  return decode(text).indexOf(tt) > -1 ? text : encode(tt + encode(text));
}
