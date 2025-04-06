import { create } from "zustand";

//TODO this type should be imported from backend in the future
export interface AppResizableLayoutStore {
  primarySideBar: {
    minWidth: number;
    width: number;
    visible: boolean;
    setWidth: (newWidth: number) => void;
    setVisible: (visible: boolean) => void;
  };
  secondarySideBar: {
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

export const useAppResizableLayoutStore = create<AppResizableLayoutStore>()((set) => ({
  primarySideBar: {
    minWidth: 270,
    width: 255,
    visible: true,
    setWidth: (newWidth) =>
      set((state) => ({
        primarySideBar: {
          ...state.primarySideBar,
          width: newWidth,
          visible: newWidth > 0,
        },
      })),
    setVisible: (visible) =>
      set((state) => ({
        primarySideBar: {
          ...state.primarySideBar,
          visible,
        },
      })),
  },
  secondarySideBar: {
    minWidth: 270,
    width: 255,
    visible: true,
    setWidth: (newWidth) =>
      set((state) => ({
        secondarySideBar: {
          ...state.secondarySideBar,
          width: newWidth,
          visible: newWidth > 0,
        },
      })),
    setVisible: (visible) =>
      set((state) => ({
        secondarySideBar: {
          ...state.secondarySideBar,
          visible,
        },
      })),
  },
  bottomPane: {
    minHeight: 100,
    height: 333,
    visible: true,
    setHeight: (newHeight) =>
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          height: newHeight,
          visible: newHeight > 0,
        },
      })),
    setVisible: (visible) =>
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          visible,
        },
      })),
  },
}));
