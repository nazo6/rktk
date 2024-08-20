import { KeyAction } from "../../bindings";
import * as kle from "@ijprest/kle-serial";

export type KeyData = {
  kleKey: kle.Key;
  changed: boolean;
  action: KeyAction;
  loc: KeyLoc;
};

export type KeyLoc = { row: number; col: number; layer: number };

export type KeyUpdate = Omit<KeyData, "kleKey" | "changed">;
