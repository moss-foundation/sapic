export type ColorValue =
  | {
      type: "solid";
      value: string;
    }
  | {
      type: "gradient";
      value: string;
    }
  | {
      type: "variable";
      value: string;
    };

type ColorStop = {
  color: string;
  percentage?: number;
};

const clamp = (min: number, max: number, value: number): number => Math.max(min, Math.min(max, value));

export const clampRgb = (value: number): number => clamp(0, 255, value);
export const clampAlpha = (value: number): number => clamp(0, 1, value);
export const clampPercent = (value: number): number => clamp(0, 100, value);

export const rgb = (r: number, g: number, b: number): string => `rgb(${clampRgb(r)}, ${clampRgb(g)}, ${clampRgb(b)})`;

export const rgba = (r: number, g: number, b: number, a: number): string =>
  `rgba(${clampRgb(r)}, ${clampRgb(g)}, ${clampRgb(b)}, ${clampAlpha(a)})`;

export type GradientDirection =
  | "to top"
  | "to bottom"
  | "to left"
  | "to right"
  | "to top right"
  | "to top left"
  | "to bottom right"
  | "to bottom left"
  | `${number}deg`;

export const linearGradient = (direction?: GradientDirection, ...colorStops: ColorStop[]): string => {
  const stops = colorStops
    .map(({ color, percentage }) => {
      if (percentage) {
        return `${color} ${clampPercent(percentage)}%`;
      } else {
        return `${color}`;
      }
    })
    .join(", ");

  if (direction) {
    return `linear-gradient(${direction}, ${stops})`;
  } else {
    return `linear-gradient(${stops})`;
  }
};
