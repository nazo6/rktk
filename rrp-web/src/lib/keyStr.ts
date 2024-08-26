import { KeyAction, KeyCode } from "rrp-client-web";
import {
  MEDIA_KEYS,
  MODIFIER_KEYS,
  MOUSE_KEYS,
  NORMAL_KEYS,
  SPECIAL_KEYS,
} from "./keys";

export function keyStr(key?: KeyAction): string {
  let str;
  if (!key) {
  } else if (key === "Inherit") {
    str = "____";
  } else if ("Normal" in key) {
    str = keyCodeStr(key.Normal);
  } else if ("Normal2" in key) {
    str = `${keyCodeStr(key.Normal2[0])} & ${keyCodeStr(key.Normal2[1])}`;
  } else if ("TapHold" in key) {
    str = `${keyCodeStr(key.TapHold[0])} / ${keyCodeStr(key.TapHold[1])}`;
  } else if ("OneShot" in key) {
    str = `OS(${keyCodeStr(key.OneShot)})`;
  } else if ("TapDance" in key) {
    str = `TD(${key.TapDance})`;
  }

  return str ?? "UNKNOWN";
}

function keyCodeStr(kc: KeyCode) {
  let str;
  if (kc == "None") {
    str = "XX";
  } else if ("Key" in kc) {
    str = NORMAL_KEYS.get(kc.Key);
  } else if ("Mouse" in kc) {
    str = MOUSE_KEYS.get(kc.Mouse);
  } else if ("Modifier" in kc) {
    str = MODIFIER_KEYS.get(kc.Modifier);
  } else if ("Layer" in kc) {
    if ("Momentary" in kc.Layer) {
      str = `MO(${kc.Layer.Momentary})`;
    } else if ("Toggle" in kc.Layer) {
      str = `TG(${kc.Layer.Toggle})`;
    }
  } else if ("Special" in kc) {
    str = SPECIAL_KEYS.get(kc.Special);
  } else if ("Media" in kc) {
    str = MEDIA_KEYS.get(kc.Media);
  }

  return str ?? "";
}
