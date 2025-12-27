import { IThemeIpc } from "@/domains/theme";
import { invoke } from "@tauri-apps/api/core";

export const themeIpc: IThemeIpc = {
  describeColorTheme: async (themeId) => {
    return await invoke("describe_color_theme", {
      input: {
        id: themeId,
      },
    });
  },
  listColorThemes: async () => {
    return await invoke("list_color_themes");
  },
};
