import type { Profile } from "./types";

export function round(number, precision) {
  var factor = Math.pow(10, precision);
  return Math.round(number * factor) / factor;
}

export function pretty_profile(p: Profile) {
  switch (p.type.toLowerCase()) {
    case "default":
      return "Default";
    case "preset":
      return `${p.power}W @ ${p.ths}`;
    case "manual":
      return `Manual ${p.freq}MHz ${p.volt}V`;
    case "lowpower":
      return `Low Power`;
  }
}