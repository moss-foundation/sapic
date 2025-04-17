import { create } from "zustand";

import { invokeTauriIpc } from "@/lib/backend/tauri";

//TODO this type should be imported from backend in the future
export interface AppResizableLayoutStore {
  sideBarPosition: "left" | "right";
  setSideBarPosition: (position: AppResizableLayoutStore["sideBarPosition"]) => void;
  sideBar: {
    minWidth: number;
    width: number;
    visible: boolean;
    setWidth: (newWidth: number) => void;
    setVisible: (visible: boolean) => void;
  };
  bottomPane: {
    minHeight: number;
    height: number;
    visible: boolean;
    setHeight: (newHeight: number) => void;
    setVisible: (visible: boolean) => void;
  };
}

export const useAppResizableLayoutStore = create<AppResizableLayoutStore>()((set, get) => ({
  sideBarPosition: "left",
  setSideBarPosition: (position: AppResizableLayoutStore["sideBarPosition"]) =>
    set(() => ({
      sideBarPosition: position,
    })),
  sideBar: {
    minWidth: 120,
    width: 255,
    visible: true,
    setWidth: (newWidth) => {
      invokeTauriIpc("set_layout_parts_state", {
        input: {
          sidebar: {
            preferredSize: newWidth,
            isVisible: newWidth > 0,
          },
        },
        params: { isOnExit: false },
      });
      set((state) => ({
        sideBar: {
          ...state.sideBar,
          width: newWidth,
          visible: newWidth > 0,
        },
      }));
    },
    setVisible: (visible) => {
      invokeTauriIpc("set_layout_parts_state", {
        input: {
          sidebar: {
            preferredSize: get().sideBar.width,
            isVisible: visible,
          },
        },
        params: { isOnExit: false },
      });
      set((state) => ({
        sideBar: {
          ...state.sideBar,
          visible,
        },
      }));
    },
  },
  bottomPane: {
    minHeight: 100,
    height: 333,
    visible: false,
    setHeight: (newHeight) => {
      invokeTauriIpc("set_layout_parts_state", {
        input: {
          panel: {
            preferredSize: newHeight,
            isVisible: newHeight > 0,
          },
        },
        params: { isOnExit: false },
      });
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          height: newHeight,
          visible: newHeight > 0,
        },
      }));
    },
    setVisible: (visible) => {
      invokeTauriIpc("set_layout_parts_state", {
        input: {
          panel: {
            preferredSize: get().bottomPane.height,
            isVisible: visible,
          },
        },
        params: { isOnExit: false },
      });
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          visible,
        },
      }));
    },
  },
}));
