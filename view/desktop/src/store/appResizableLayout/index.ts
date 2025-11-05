import { create } from "zustand";

import { SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { sharedStorageService } from "@/lib/services";
import { SidebarPosition } from "@repo/moss-workspace";

//TODO this type should be imported from backend in the future
export interface AppResizableLayoutStore {
  initialize: (workspaceId: string) => Promise<void>;

  sideBarPosition: SidebarPosition;
  setSideBarPosition: (position: SidebarPosition, workspaceId: string) => void;

  sideBar: {
    minWidth: number;
    maxWidth: number;
    width: number;
    visible: boolean;
    setWidth: (newWidth: number, workspaceId: string) => void;
    setVisible: (visible: boolean, workspaceId: string) => void;
  };
  bottomPane: {
    minHeight: number;
    maxHeight: number;
    height: number;
    visible: boolean;
    setHeight: (newHeight: number, workspaceId: string) => void;
    setVisible: (visible: boolean, workspaceId: string) => void;
  };
}

const defaultSidebarPosition = SIDEBAR_POSITION.LEFT as SidebarPosition;

const defaultSidebarSize = {
  width: 255,
  visible: true,
} as const;

const defaultBottomPaneSize = {
  height: 333,
  visible: false,
} as const;

export const useAppResizableLayoutStore = create<AppResizableLayoutStore>()((set, get) => ({
  sideBarPosition: SIDEBAR_POSITION.LEFT,
  setSideBarPosition: (position: SidebarPosition, workspaceId: string) => {
    sharedStorageService.putItem("sidebarPosition", position, workspaceId);
    set(() => {
      return {
        sideBarPosition: position,
      };
    });
  },
  initialize: async (workspaceId) => {
    const sidebarPosition = (await sharedStorageService.getItem("sidebarPosition", workspaceId))?.value;
    const sidebarWidth = (await sharedStorageService.getItem("sidebarWidth", workspaceId))?.value;
    const sidebarVisible = (await sharedStorageService.getItem("sidebarVisible", workspaceId))?.value;
    const bottomPaneHeight = (await sharedStorageService.getItem("bottomPaneHeight", workspaceId))?.value;
    const bottomPaneVisible = (await sharedStorageService.getItem("bottomPaneVisible", workspaceId))?.value;

    set((state) => {
      return {
        sideBarPosition: sidebarPosition ?? defaultSidebarPosition,
        sideBar: {
          ...state.sideBar,
          width: sidebarWidth ?? defaultSidebarSize.width,
          visible: sidebarVisible ?? defaultSidebarSize.visible,
        },
        bottomPane: {
          ...state.bottomPane,
          height: bottomPaneHeight ?? defaultBottomPaneSize.height,
          visible: bottomPaneVisible ?? defaultBottomPaneSize.visible,
        },
      };
    });
  },
  sideBar: {
    minWidth: 130,
    maxWidth: 400,
    width: 255,
    visible: true,
    setWidth: (newWidth, workspaceId) => {
      sharedStorageService.putItem("sidebarWidth", newWidth, workspaceId);
      set((state) => {
        return {
          sideBar: {
            ...state.sideBar,
            width: newWidth <= 0 ? get().sideBar.width : newWidth,
            visible: newWidth > 0,
          },
        };
      });
    },
    setVisible: (visible, workspaceId) => {
      sharedStorageService.putItem("sidebarVisible", visible, workspaceId);
      set((state) => {
        return {
          sideBar: {
            ...state.sideBar,
            visible,
          },
        };
      });
    },
  },
  bottomPane: {
    minHeight: 100,
    maxHeight: Infinity,
    height: 333,
    visible: false,
    setHeight: (newHeight, workspaceId) => {
      sharedStorageService.putItem("bottomPaneHeight", newHeight, workspaceId);
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          height: newHeight,
          visible: newHeight > 0,
        },
      }));
    },
    setVisible: (visible, workspaceId) => {
      sharedStorageService.putItem("bottomPaneVisible", visible, workspaceId);
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          visible,
        },
      }));
    },
  },
}));
