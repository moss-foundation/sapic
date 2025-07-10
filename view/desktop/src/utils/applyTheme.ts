import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { GetColorThemeInput, GetColorThemeOutput } from "@repo/moss-app";
import { QueryClient } from "@tanstack/react-query";

interface ThemeStore {
  shouldApplyTheme(themeId: string): boolean;
  setIsApplying(isApplying: boolean): void;
  setCurrentThemeId(themeId: string): void;
}

// Legacy direct API call (used by non-React contexts)
export const getColorTheme = async (input: GetColorThemeInput): Promise<IpcResult<GetColorThemeOutput, string>> => {
  return await invokeTauriIpc("get_color_theme", {
    input: input,
  });
};

let currentAppliedThemeId: string | null = null;

export const applyColorTheme = async (themeId: string): Promise<void> => {
  try {
    if (currentAppliedThemeId === themeId) {
      return;
    }

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
    currentAppliedThemeId = themeId;
  } catch (error) {
    console.error(`Failed to apply theme "${themeId}":`, error);
  }
};

// Cached version that uses React Query client to avoid duplicate API calls
export const applyColorThemeFromCache = async (
  themeId: string,
  queryClient: QueryClient,
  themeStore?: ThemeStore
): Promise<void> => {
  try {
    if (themeStore && !themeStore.shouldApplyTheme(themeId)) {
      return;
    }

    if (!themeStore && currentAppliedThemeId === themeId) {
      return;
    }

    // Mark as applying to prevent concurrent applications
    if (themeStore) {
      themeStore.setIsApplying(true);
    }

    const cachedTheme = queryClient.getQueryData<GetColorThemeOutput>(["getColorTheme", themeId]);

    let cssContent: string;

    if (cachedTheme) {
      cssContent = cachedTheme.cssContent;
    } else {
      const getColorThemeOutput = await getColorTheme({ id: themeId });
      if (getColorThemeOutput.status !== "ok") {
        console.error(`Error reading theme file for "${themeId}":`, getColorThemeOutput.error);
        if (themeStore) themeStore.setIsApplying(false);
        return;
      }
      cssContent = getColorThemeOutput.data.cssContent;

      queryClient.setQueryData(["getColorTheme", themeId], getColorThemeOutput.data);
    }

    let styleTag = document.getElementById("theme-style") as HTMLStyleElement | null;

    if (!styleTag) {
      styleTag = document.createElement("style");
      styleTag.id = "theme-style";
      document.head.appendChild(styleTag);
    }

    styleTag.innerHTML = cssContent;

    currentAppliedThemeId = themeId;
    if (themeStore) {
      themeStore.setCurrentThemeId(themeId);
      themeStore.setIsApplying(false);
    }
  } catch (error) {
    console.error(`Failed to apply theme "${themeId}":`, error);
    if (themeStore) themeStore.setIsApplying(false);
  }
};
