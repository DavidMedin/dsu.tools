import {
  defaults,
  ColorSpace,
  HCT,
  A98RGB,
  HSL,
  HSLuv,
  HSV,
  OKLab,
  OKLCH,
  P3,
  sRGB,
  sRGB_Linear,
  toGamut,
  to as convert,
} from "colorjs.io/fn";

export function truncToTwoDecimalPlaces(n) {
  return Math.floor(n * 100) / 100;
}

export function fmt_convert(color, space) {
  return toGamut(convert(color, space)).coords.map((n) =>
    truncToTwoDecimalPlaces(n),
  );
}

export function to_hex(color) {
  let rgb_color = fmt_convert(color, sRGB_Linear);
  return (
    "#" +
    rgb_color
      .map((c) =>
        Math.round(c * 255)
          .toString(16)
          .padStart(2, "0"),
      )
      .join("")
  );
}
