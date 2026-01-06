import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";
import { defaultLayoutState } from "@/workbench/domains/layout/defaults";
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
    const { value: output } = await sharedStorageIpc.getItem(SHARED_STORAGE_LAYOUT_KEY, {
      workspace: workspaceId ?? "application",
    });

    if (output !== "none") {
      return output.value as unknown as LayoutStateOutput;
    }

    return defaultLayoutState;
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
