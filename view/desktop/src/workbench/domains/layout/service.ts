import { defaultLayoutState } from "@/defaults/layout";
import { sharedStorageIpc } from "@/infra/ipc/sharedStorage";
import { JsonValue } from "@repo/moss-bindingutils";

import { ActivitybarState, BottomPanelState, SidebarState, TabbedPaneState } from "./types";

const SHARED_STORAGE_LAYOUT_KEY = "workbench.layout" as const;

interface ILayoutService {
  getLayout: (workspaceId: string) => Promise<LayoutStateOutput>;
  updateLayout: (input: LayoutStateInput, workspaceId: string) => Promise<void>;
  removeLayout: (workspaceId: string) => Promise<void>;
}

export const layoutService: ILayoutService = {
  getLayout: async (workspaceId: string) => {
    try {
      const { value } = await sharedStorageIpc.getItem(SHARED_STORAGE_LAYOUT_KEY, {
        workspace: workspaceId ?? "application",
      });
      return value as unknown as LayoutStateOutput;
    } catch (error) {
      console.error("Failed to get layout", error);
      return defaultLayoutState;
    }
  },
  updateLayout: async (input, workspaceId) => {
    await sharedStorageIpc.putItem(SHARED_STORAGE_LAYOUT_KEY, input as unknown as JsonValue, {
      workspace: workspaceId ?? "application",
    });
  },
  removeLayout: async (workspaceId: string) => {
    await sharedStorageIpc.removeItem(SHARED_STORAGE_LAYOUT_KEY, {
      workspace: workspaceId ?? "application",
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
