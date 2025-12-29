import { GetColorThemeOutput, ListColorThemesOutput } from "@repo/ipc";

export interface IThemeIpc {
  describeColorTheme: (themeId: string) => Promise<GetColorThemeOutput>;

  listColorThemes: () => Promise<ListColorThemesOutput>;
}
