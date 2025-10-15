import { invokeTauriServiceIpc } from "@/lib/backend/tauri";
import { GetColorThemeInput, GetColorThemeOutput, ListColorThemesOutput } from "@repo/moss-app";

export const themesService = {
  describeColorTheme: async (themeId: string) => {
    return await invokeTauriServiceIpc<GetColorThemeInput, GetColorThemeOutput>({
      cmd: "describe_color_theme",
      args: {
        input: { id: themeId },
      },
    });
  },

  listColorThemes: async () => {
    return await invokeTauriServiceIpc<void, ListColorThemesOutput>({ cmd: "list_color_themes" });
  },
};
