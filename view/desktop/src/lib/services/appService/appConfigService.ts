import { invokeTauriServiceIpc } from "@/lib/backend/tauri";
import { DescribeAppOutput, UpdateConfigurationInput } from "@repo/moss-app";
import {
  ActivitybarPartStateInfo,
  EditorPartStateInfo,
  PanelPartStateInfo,
  SidebarPartStateInfo,
  UpdateLayoutInput,
} from "@repo/moss-workspace";

export const appConfigService = {
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
};
