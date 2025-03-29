import { create } from "zustand";

//TODO this type should be imported from backend in the future
export interface AppResizableLayoutStore {
  sideBar: {
    width: number;
    setWidth: (newWidth: number) => void;
    getWidth: () => number;
  };
  bottomPane: {
    height: number;
    visibility: boolean;
    setHeight: (newHeight: number) => void;
    setVisibility: (visibility: boolean) => void;
    getHeight: () => number;
  };
}

export const useAppResizableLayoutStore = create<AppResizableLayoutStore>()((set, get) => ({
  sideBar: {
    width: 340,
    setWidth: (newWidth) =>
      set((state) => ({
        sideBar: {
          ...state.sideBar,
          width: newWidth,
        },
      })),
    getWidth: () => {
      return get().sideBar.width;
    },
  },
  bottomPane: {
    height: 333,
    visibility: true,
    setHeight: (newHeight) =>
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          height: newHeight,
          visibility: newHeight > 0,
        },
      })),
    setVisibility: (visibility) =>
      set((state) => ({
        bottomPane: {
          ...state.bottomPane,
          visibility,
        },
      })),
    getHeight: () => {
      return get().bottomPane.height;
    },
  },
}));
