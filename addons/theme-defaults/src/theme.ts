import { ColorValue } from "./color";

export type ThemeMode = "light" | "dark";

export type Theme = {
  identifier: string;
  displayName: string;
  mode: ThemeMode;
  palette: { [key in string]?: ColorValue };
  colors: { [key in string]?: ColorValue };
  boxShadows: { [key in string]?: string };
};
