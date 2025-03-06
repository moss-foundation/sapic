import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { ListLocalesOutput } from "@repo/moss-nls";
import { DescribeAppStateOutput } from "@repo/moss-state";
import { ListThemesOutput } from "@repo/moss-theme";
import { invoke } from "@tauri-apps/api/core";

// App state

export const describeAppState = async (): Promise<DescribeAppStateOutput> => {
  return await invoke<DescribeAppStateOutput>("describe_app_state");
};

//Color themes

export const listThemes = async (): Promise<ListThemesOutput> => {
  return await invoke<ListThemesOutput>("list_themes");
};

export const getColorTheme = async (source: string): Promise<IpcResult<string, string>> => {
  return await invokeTauriIpc("get_color_theme", {
    path: source,
  });
};

//Language packs

export const listLocales = async (): Promise<ListLocalesOutput> => {
  return await invoke<ListLocalesOutput>("list_locales");
};
