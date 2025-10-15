import {
  DescribeAppOutput,
  GetColorThemeInput,
  GetColorThemeOutput,
  ListColorThemesOutput,
  ListLocalesOutput,
  UpdateConfigurationInput,
} from "@repo/moss-app";
import {
  ActivitybarPartStateInfo,
  EditorPartStateInfo,
  PanelPartStateInfo,
  SidebarPartStateInfo,
  UpdateLayoutInput,
} from "@repo/moss-workspace";

import { invokeTauriServiceIpc } from "../backend/tauri";

//FIXME services should take only a Input types ideally
export const AppService = {
  describeApp: async () => {
    return await invokeTauriServiceIpc<void, DescribeAppOutput>({ cmd: "describe_app" });
  },

  updateActivitybarPartState: async (activitybar: ActivitybarPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateLayoutInput, void>({
      cmd: "update_layout",
      args: {
        input: { activitybar },
      },
    });
  },

  updateEditorPartState: async (editor: EditorPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateLayoutInput, void>({
      cmd: "update_layout",
      args: {
        input: { editor },
      },
    });
  },

  updatePanelPartState: async (panel: PanelPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateLayoutInput, void>({
      cmd: "update_layout",
      args: {
        input: { panel },
      },
    });
  },

  updateSidebarPartState: async (sidebar: SidebarPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateLayoutInput, void>({
      cmd: "update_layout",
      args: {
        input: { sidebar },
      },
    });
  },

  updateConfiguration: async (configuration: UpdateConfigurationInput) => {
    return await invokeTauriServiceIpc<UpdateConfigurationInput, void>({
      cmd: "update_configuration",
      args: {
        input: {
          ...configuration,
        },
      },
    });
  },

  listLocales: async () => {
    return await invokeTauriServiceIpc<void, ListLocalesOutput>({ cmd: "list_locales" });
  },

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
