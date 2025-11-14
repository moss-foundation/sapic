import { invokeTauriServiceIpc } from "@/infra/ipc/tauri";
import { GetColorThemeInput, GetColorThemeOutput, ListColorThemesOutput } from "@repo/ipc";

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
