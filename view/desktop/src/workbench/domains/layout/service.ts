import { sharedStorageIpc } from "@/infra/ipc/sharedStorage";
import { JsonValue } from "@repo/moss-bindingutils";

import { ActivitybarState, BottomPanelState, SidebarState, TabbedPaneState } from "./types";

const SHARED_STORAGE_LAYOUT_KEY = "workbench.layout" as const;

export const layoutService = {
  getLayout: async (workspaceId: string) => {
    const { value } = await sharedStorageIpc.getItem(SHARED_STORAGE_LAYOUT_KEY, {
      workspace: workspaceId,
    });
    return value as unknown as LayoutStateOutput;
  },
  updateLayout: async (input: LayoutStateInput, workspaceId: string) => {
    return await sharedStorageIpc.putItem(SHARED_STORAGE_LAYOUT_KEY, input as unknown as JsonValue, {
      workspace: workspaceId,
    });
  },
  removeLayout: async (workspaceId: string) => {
    return await sharedStorageIpc.removeItem(SHARED_STORAGE_LAYOUT_KEY, {
      workspace: workspaceId,
    });
  },
};

export interface LayoutStateOutput {
  sidebarState: SidebarState;
  bottomPanelState: BottomPanelState;
  tabbedPaneState: TabbedPaneState;
  activitybarState: ActivitybarState;
}

type DeepPartial<T> = T extends object ? { [K in keyof T]?: DeepPartial<T[K]> } : T;
type Simplify<T> = { [K in keyof T]: T[K] } & {};

export type LayoutStateInput = Simplify<DeepPartial<LayoutStateOutput>>;
