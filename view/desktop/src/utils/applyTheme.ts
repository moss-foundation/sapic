import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";

export const getColorTheme = async (id: string): Promise<IpcResult<string, string>> => {
  return await invokeTauriIpc("get_color_theme", {
    id: id,
  });
};

export const applyTheme = async (id: string) => {
  try {
    const result: IpcResult<string, string> = await getColorTheme(id);
    if (result.status === "ok") {
      const cssContent = result.data;
      let styleTag = document.getElementById("theme-style") as HTMLStyleElement | null;

      if (styleTag) {
        styleTag.innerHTML = cssContent;
      } else {
        styleTag = document.createElement("style");
        styleTag.id = "theme-style";
        styleTag.innerHTML = cssContent;
        document.head.appendChild(styleTag);
      }
    } else {
      console.error(`Error reading theme file for "${id}":`, result.error);
    }
  } catch (error) {
    console.error(`Failed to apply theme "${id}":`, error);
  }
};
