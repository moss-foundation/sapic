import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { GetColorThemeInput, GetColorThemeOutput } from "@repo/moss-theme";

export const getColorTheme = async (input: GetColorThemeInput): Promise<IpcResult<GetColorThemeOutput, string>> => {
  return await invokeTauriIpc("get_color_theme", {
    input: input,
  });
};

export const applyColorTheme = async (themeId: string): Promise<void> => {
  try {
    const getColorThemeOutput: IpcResult<GetColorThemeOutput, string> = await getColorTheme({
      id: themeId,
    });
    if (getColorThemeOutput.status !== "ok") {
      console.error(`Error reading theme file for "${themeId}":`, getColorThemeOutput.error);
      return;
    }

    const cssContent = getColorThemeOutput.data.cssContent;
    let styleTag = document.getElementById("theme-style") as HTMLStyleElement | null;

    if (!styleTag) {
      styleTag = document.createElement("style");
      styleTag.id = "theme-style";
      document.head.appendChild(styleTag);
    }

    styleTag.innerHTML = cssContent;
  } catch (error) {
    console.error(`Failed to apply theme "${themeId}":`, error);
  }
};
