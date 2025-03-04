import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { DescribeAppStateOutput } from "@repo/moss-state";
import { ListThemesOutput } from "@repo/moss-theme";
import { invoke } from "@tauri-apps/api/core";

// App state

export const getState = async (): Promise<DescribeAppStateOutput> => {
  return await invoke<DescribeAppStateOutput>("describe_app_state");
};

//Color themes

export const getColorThemes = async (): Promise<ListThemesOutput> => {
  return await invoke<ListThemesOutput>("list_themes");
};

export const getColorTheme = async (source: string): Promise<IpcResult<string, string>> => {
  return await invokeTauriIpc("get_color_theme", {
    path: source,
  });
};

//Language packs
/*
export const getLanguagePacks = async (): Promise<LocaleDescriptor[]> => {
  return await invoke<LocaleDescriptor[]>("get_locales");
};

//Activities

export const getAllActivities = async (): Promise<IpcResult<MenuItem[], Error>> => {
  return await invokeTauriIpc("get_menu_items_by_namespace", { namespace: "headItem" }); // this here should be a type
};
*/
