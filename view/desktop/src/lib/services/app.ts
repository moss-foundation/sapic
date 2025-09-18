import { DescribeAppOutput } from "@repo/moss-app";
import {
  ActivitybarPartStateInfo,
  EditorPartStateInfo,
  PanelPartStateInfo,
  SidebarPartStateInfo,
  UpdateStateInput,
} from "@repo/moss-workspace";

import { invokeTauriServiceIpc } from "../backend/tauri";

//FIXME services should take only a Input types ideally
export const AppService = {
  describeApp: async () => {
    return await invokeTauriServiceIpc<void, DescribeAppOutput>({ cmd: "describe_app" });
  },

  updateActivitybarPartState: async (activitybar: ActivitybarPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateStateInput, void>({
      cmd: "update_workspace_state",
      args: {
        input: { "updateActivitybarPartState": activitybar },
      },
    });
  },

  updateEditorPartState: async (editor: EditorPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateStateInput, void>({
      cmd: "update_workspace_state",
      args: {
        input: { "updateEditorPartState": editor },
      },
    });
  },

  updatePanelPartState: async (panel: PanelPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateStateInput, void>({
      cmd: "update_workspace_state",
      args: {
        input: { "updatePanelPartState": panel },
      },
    });
  },

  updateSidebarPartState: async (sidebar: SidebarPartStateInfo) => {
    return await invokeTauriServiceIpc<UpdateStateInput, void>({
      cmd: "update_workspace_state",
      args: {
        input: { "updateSidebarPartState": sidebar },
      },
    });
  },
};
