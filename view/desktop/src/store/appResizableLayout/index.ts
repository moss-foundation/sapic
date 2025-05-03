import { create } from "zustand";

//TODO this type should be imported from backend in the future
export interface AppResizableLayoutStore {
  sideBarPosition: "left" | "right";
  setSideBarPosition: (position: AppResizableLayoutStore["sideBarPosition"]) => void;
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

export const useAppResizableLayoutStore = create<AppResizableLayoutStore>()((set, get) => ({
  sideBarPosition: "left",
  setSideBarPosition: (position: AppResizableLayoutStore["sideBarPosition"]) =>
    set(() => ({
      sideBarPosition: position,
    })),
  initialize: (newState) => {
    set((state) => ({
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
    }));
  },
  sideBar: {
    minWidth: 130,
    maxWidth: 400,
    width: 255,
    visible: true,
    setWidth: (newWidth) => {
      set((state) => ({
        sideBar: {
          ...state.sideBar,
          width: newWidth <= 0 ? get().sideBar.width : newWidth,
          visible: newWidth > 0,
        },
      }));
    },
    setVisible: (visible) => {
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
