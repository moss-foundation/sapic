import { Config } from "tailwindcss";
import tailwindAnimate from "tailwindcss-animate";

import breakpoints from "./extends/breakpoints";
import fontSize from "./extends/fontSize";
import plugins from "./extends/plugins";
import typography from "./extends/typography";

const config: Config = {
  theme: {
    extend: {
      fontSize: fontSize,
      fontFamily: typography,
      screens: breakpoints,
    },
  },
  darkMode: ["class", '[data-theme="dark"]'],
  plugins: [tailwindAnimate, ...plugins],
};

export default config;
