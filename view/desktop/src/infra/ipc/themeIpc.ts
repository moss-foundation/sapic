import { IThemeIpc } from "@/domains/theme";

import { invokeTauriServiceIpc } from "./tauri";

export const themeIpc: IThemeIpc = {
  describeColorTheme: async (themeId) => {
    return await invokeTauriServiceIpc("describe_color_theme", {
      input: {
        id: themeId,
      },
    });
  },
  listColorThemes: async () => {
    return await invokeTauriServiceIpc("list_color_themes");
  },
};
