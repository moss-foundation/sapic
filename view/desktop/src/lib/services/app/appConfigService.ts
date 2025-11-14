import { invokeTauriServiceIpc } from "@/infra/ipc/tauri";
import {
  ActivitybarPartStateInfo,
  EditorPartStateInfo,
  PanelPartStateInfo,
  SidebarPartStateInfo,
  UpdateLayoutInput,
} from "@repo/moss-workspace";
import { DescribeAppOutput, UpdateConfigurationInput } from "@repo/window";

export const appConfigService = {
  describeApp: async () => {
    return await invokeTauriServiceIpc<void, DescribeAppOutput>({ cmd: "describe_app" });
  },

  //TODO describe app changed, we dont need to update activitybar part state
  updateActivitybarPartState: async (activitybar: ActivitybarPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateLayoutInput, void>({
      cmd: "update_layout",
      args: {
        input: { activitybar },
      },
    });
  },

  //TODO describe app changed, we dont need to update editor part state
  updateEditorPartState: async (editor: EditorPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateLayoutInput, void>({
      cmd: "update_layout",
      args: {
        input: { editor },
      },
    });
  },

  //TODO describe app changed, we dont need to update panel part state
  updatePanelPartState: async (panel: PanelPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateLayoutInput, void>({
      cmd: "update_layout",
      args: {
        input: { panel },
      },
    });
  },

  //TODO describe app changed, we dont need to update sidebar part state
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
