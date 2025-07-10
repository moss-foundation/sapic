import { create } from "zustand";

interface ThemeState {
  currentThemeId: string | null;
  isApplying: boolean;
  setCurrentThemeId: (themeId: string | null) => void;
  setIsApplying: (isApplying: boolean) => void;
  shouldApplyTheme: (themeId: string) => boolean;
}

export const useThemeStore = create<ThemeState>((set, get) => ({
  currentThemeId: null,
  isApplying: false,
  setCurrentThemeId: (themeId: string | null) => set({ currentThemeId: themeId }),
  setIsApplying: (isApplying: boolean) => set({ isApplying }),
  shouldApplyTheme: (themeId: string) => {
    const state = get();
    // Don't apply if same theme or currently applying
    return state.currentThemeId !== themeId && !state.isApplying;
  },
}));
