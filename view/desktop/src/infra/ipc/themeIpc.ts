import { IThemeIpc } from "@/domains/theme";

import { invokeTauriIpc } from "./tauri";

export const themeIpc: IThemeIpc = {
  describeColorTheme: async (themeId) => {
    return await invokeTauriIpc("describe_color_theme", {
      input: {
        id: themeId,
      },
    });
  },
  listColorThemes: async () => {
    return await invokeTauriIpc("list_color_themes");
  },
};
