import { create } from "zustand";

import { setLayoutPartsState } from "@/utils/setLayoutPartsState";

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
      setLayoutPartsState({
        input: {
          sidebar: {
            preferredSize: newWidth <= 0 ? get().sideBar.width : newWidth,
            isVisible: newWidth > 0,
          },
        },
      });
      set((state) => ({
        sideBar: {
          ...state.sideBar,
          width: newWidth <= 0 ? get().sideBar.width : newWidth,
          visible: newWidth > 0,
        },
      }));
    },
    setVisible: (visible) => {
      setLayoutPartsState({
        input: {
          sidebar: {
            preferredSize: get().sideBar.width,
            isVisible: visible,
          },
        },
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
      setLayoutPartsState({
        input: {
          panel: {
            preferredSize: newHeight,
            isVisible: newHeight > 0,
          },
        },
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
      setLayoutPartsState({
        input: {
          panel: {
            preferredSize: get().bottomPane.height,
            isVisible: visible,
          },
        },
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
