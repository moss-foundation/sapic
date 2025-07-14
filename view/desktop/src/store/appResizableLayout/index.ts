import { create } from "zustand";

import { SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { SidebarPosition } from "@repo/moss-workspace";

//TODO this type should be imported from backend in the future
export interface AppResizableLayoutStore {
  sideBarPosition: SidebarPosition;
  setSideBarPosition: (position: SidebarPosition) => void;
  initialize: (newState: {
    sideBar: {
      width?: number;
      visible?: boolean;
    };
    bottomPane: {
      height?: number;
      visible?: boolean;
    };
  }) => void;

  sideBar: {
    minWidth: number;
    maxWidth: number;
    width: number;
    visible: boolean;
    setWidth: (newWidth: number) => void;
    setVisible: (visible: boolean) => void;
  };
  bottomPane: {
    minHeight: number;
    maxHeight: number;
    height: number;
    visible: boolean;
    setHeight: (newHeight: number) => void;
    setVisible: (visible: boolean) => void;
  };
}

let userHasChangedSidebar = false;
let initialSidebarWidth: number | null = null;

export const useAppResizableLayoutStore = create<AppResizableLayoutStore>()((set, get) => ({
  sideBarPosition: SIDEBAR_POSITION.LEFT,
  setSideBarPosition: (position: SidebarPosition) =>
    set(() => {
      userHasChangedSidebar = true;
      return {
        sideBarPosition: position,
      };
    }),
  initialize: (newState) => {
    set((state) => {
      // If user has changed the sidebar and we're trying to initialize the sidebar, skip it
      if (userHasChangedSidebar && (newState.sideBar.width !== undefined || newState.sideBar.visible !== undefined)) {
        return {
          sideBar: {
            ...state.sideBar,
            // Keep existing width and visibility, don't override with initialization
          },
          bottomPane: {
            ...state.bottomPane,
            height: newState.bottomPane.height ?? state.bottomPane.height,
            visible: newState.bottomPane.visible ?? state.bottomPane.visible,
          },
        };
      }

      // Store the initial width for comparison
      if (initialSidebarWidth === null && newState.sideBar.width !== undefined) {
        initialSidebarWidth = newState.sideBar.width;
      }

      return {
        sideBar: {
          ...state.sideBar,
          width: newState.sideBar.width ?? state.sideBar.width,
          visible: newState.sideBar.visible ?? state.sideBar.visible,
        },
        bottomPane: {
          ...state.bottomPane,
          height: newState.bottomPane.height ?? state.bottomPane.height,
          visible: newState.bottomPane.visible ?? state.bottomPane.visible,
        },
      };
    });
  },
  sideBar: {
    minWidth: 130,
    maxWidth: 400,
    width: 255,
    visible: true,
    setWidth: (newWidth) => {
      set((state) => {
        // Mark that user has changed sidebar if they changed width from initial
        if (initialSidebarWidth !== null && newWidth !== initialSidebarWidth) {
          userHasChangedSidebar = true;
        }

        return {
          sideBar: {
            ...state.sideBar,
            width: newWidth <= 0 ? get().sideBar.width : newWidth,
            visible: newWidth > 0,
          },
        };
      });
    },
    setVisible: (visible) => {
      set((state) => {
        userHasChangedSidebar = true;
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
    setHeight: (newHeight) => {
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          height: newHeight,
          visible: newHeight > 0,
        },
      }));
    },
    setVisible: (visible) => {
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          visible,
        },
      }));
    },
  },
}));
