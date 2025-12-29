import { themeIpc } from "@/infra/ipc/themeIpc";
import { GetColorThemeOutput, ListColorThemesOutput } from "@repo/ipc";

interface IThemeService {
  describeColorTheme: (themeId: string) => Promise<GetColorThemeOutput>;
  listColorThemes: () => Promise<ListColorThemesOutput>;
}

export const themeService: IThemeService = {
  describeColorTheme: async (themeId) => {
    return await themeIpc.describeColorTheme(themeId);
  },
  listColorThemes: async () => {
    return await themeIpc.listColorThemes();
  },
};
