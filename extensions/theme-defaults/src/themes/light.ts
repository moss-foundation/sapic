import { rgba } from "../color";
import { Theme } from "../theme";

export const defaultLightTheme: Theme = {
  identifier: "moss.sapic-theme.lightDefault",
  displayName: "Light Default",
  mode: "light",
  palette: {
    "moss.gray.1": {
      type: "solid",
      value: "#000000",
    },
    "moss.gray.2": {
      type: "solid",
      value: "#27282e",
    },
    "moss.gray.3": {
      type: "solid",
      value: "#383a42",
    },
    "moss.gray.4": {
      type: "solid",
      value: "#494b57",
    },
    "moss.gray.5": {
      type: "solid",
      value: "#5a5d6b",
    },
    "moss.gray.6": {
      type: "solid",
      value: "#6c707e",
    },
    "moss.gray.7": {
      type: "solid",
      value: "#818594",
    },
    "moss.gray.8": {
      type: "solid",
      value: "#a8adbd",
    },
    "moss.gray.9": {
      type: "solid",
      value: "#c9ccd6",
    },
    "moss.gray.10": {
      type: "solid",
      value: "#d3d5db",
    },
    "moss.gray.11": {
      type: "solid",
      value: "#dfe1e5",
    },
    "moss.gray.12": {
      type: "solid",
      value: "#ebecf0",
    },
    "moss.gray.13": {
      type: "solid",
      value: "#f7f8fa",
    },
    "moss.gray.14": {
      type: "solid",
      value: "#ffffff",
    },
    "moss.blue.1": {
      type: "solid",
      value: "#2e55a3",
    },
    "moss.blue.2": {
      type: "solid",
      value: "#315fbd",
    },
    "moss.blue.3": {
      type: "solid",
      value: "#3369d6",
    },
    "moss.blue.4": {
      type: "solid",
      value: "#3574f0",
    },
    "moss.blue.5": {
      type: "solid",
      value: "#4682fa",
    },
    "moss.blue.6": {
      type: "solid",
      value: "#588cf3",
    },
    "moss.blue.7": {
      type: "solid",
      value: "#709cf5",
    },
    "moss.blue.8": {
      type: "solid",
      value: "#88adf7",
    },
    "moss.blue.9": {
      type: "solid",
      value: "#a0bdf8",
    },
    "moss.blue.10": {
      type: "solid",
      value: "#c2d6fc",
    },
    "moss.blue.11": {
      type: "solid",
      value: "#d4e2ff",
    },
    "moss.blue.12": {
      type: "solid",
      value: "#edf3ff",
    },
    "moss.blue.13": {
      type: "solid",
      value: "#f5f8fe",
    },
    "moss.green.1": {
      type: "solid",
      value: "#1e6b33",
    },
    "moss.green.2": {
      type: "solid",
      value: "#1f7536",
    },
    "moss.green.3": {
      type: "solid",
      value: "#1f8039",
    },
    "moss.green.4": {
      type: "solid",
      value: "#208a3c",
    },
    "moss.green.5": {
      type: "solid",
      value: "#369650",
    },
    "moss.green.6": {
      type: "solid",
      value: "#55a76a",
    },
    "moss.green.7": {
      type: "solid",
      value: "#89c398",
    },
    "moss.green.8": {
      type: "solid",
      value: "#afdbb8",
    },
    "moss.green.9": {
      type: "solid",
      value: "#c5e5cc",
    },
    "moss.green.10": {
      type: "solid",
      value: "#e6f7e9",
    },
    "moss.green.11": {
      type: "solid",
      value: "#f2fcf3",
    },
    "moss.red.1": {
      type: "solid",
      value: "#ad2b38",
    },
    "moss.red.2": {
      type: "solid",
      value: "#bc303e",
    },
    "moss.red.3": {
      type: "solid",
      value: "#cc3645",
    },
    "moss.red.4": {
      type: "solid",
      value: "#db3b4b",
    },
    "moss.red.5": {
      type: "solid",
      value: "#e55765",
    },
    "moss.red.6": {
      type: "solid",
      value: "#e46a76",
    },
    "moss.red.7": {
      type: "solid",
      value: "#ed99a1",
    },
    "moss.red.8": {
      type: "solid",
      value: "#f2b6bb",
    },
    "moss.red.9": {
      type: "solid",
      value: "#fad4d8",
    },
    "moss.red.10": {
      type: "solid",
      value: "#fff2f3",
    },
    "moss.red.11": {
      type: "solid",
      value: "#fff7f7",
    },
    "moss.yellow.1": {
      type: "solid",
      value: "#a46704",
    },
    "moss.yellow.2": {
      type: "solid",
      value: "#c27d04",
    },
    "moss.yellow.3": {
      type: "solid",
      value: "#df9303",
    },
    "moss.yellow.4": {
      type: "solid",
      value: "#ffaf0f",
    },
    "moss.yellow.5": {
      type: "solid",
      value: "#fdbd3d",
    },
    "moss.yellow.6": {
      type: "solid",
      value: "#fed277",
    },
    "moss.yellow.7": {
      type: "solid",
      value: "#fee6b1",
    },
    "moss.yellow.8": {
      type: "solid",
      value: "#fff1d1",
    },
    "moss.yellow.9": {
      type: "solid",
      value: "#fff6de",
    },
    "moss.yellow.10": {
      type: "solid",
      value: "#fffaeb",
    },
    "moss.orange.1": {
      type: "solid",
      value: "#a14916",
    },
    "moss.orange.2": {
      type: "solid",
      value: "#b85516",
    },
    "moss.orange.3": {
      type: "solid",
      value: "#ce6117",
    },
    "moss.orange.4": {
      type: "solid",
      value: "#e56d17",
    },
    "moss.orange.5": {
      type: "solid",
      value: "#ec8f4c",
    },
    "moss.orange.6": {
      type: "solid",
      value: "#f2b181",
    },
    "moss.orange.7": {
      type: "solid",
      value: "#f9d2b6",
    },
    "moss.orange.8": {
      type: "solid",
      value: "#fce6d6",
    },
    "moss.orange.9": {
      type: "solid",
      value: "#fff4eb",
    },
    "moss.purple.1": {
      type: "solid",
      value: "#55339c",
    },
    "moss.purple.2": {
      type: "solid",
      value: "#643cb8",
    },
    "moss.purple.3": {
      type: "solid",
      value: "#7444d4",
    },
    "moss.purple.4": {
      type: "solid",
      value: "#834df0",
    },
    "moss.purple.5": {
      type: "solid",
      value: "#a177f4",
    },
    "moss.purple.6": {
      type: "solid",
      value: "#bfa1f8",
    },
    "moss.purple.7": {
      type: "solid",
      value: "#dccbfb",
    },
    "moss.purple.8": {
      type: "solid",
      value: "#efe5ff",
    },
    "moss.purple.9": {
      type: "solid",
      value: "#faf5ff",
    },
    "moss.teal.1": {
      type: "solid",
      value: "#096a6e",
    },
    "moss.teal.2": {
      type: "solid",
      value: "#077a7f",
    },
    "moss.teal.3": {
      type: "solid",
      value: "#058b90",
    },
    "moss.teal.4": {
      type: "solid",
      value: "#039ba1",
    },
    "moss.teal.5": {
      type: "solid",
      value: "#3fb3b8",
    },
    "moss.teal.6": {
      type: "solid",
      value: "#7bcccf",
    },
    "moss.teal.7": {
      type: "solid",
      value: "#b6e4e5",
    },
    "moss.teal.8": {
      type: "solid",
      value: "#daf4f5",
    },
    "moss.teal.9": {
      type: "solid",
      value: "#f2fcfc",
    },
  },
  colors: {
    //general
    "moss.primary": { type: "variable", value: "moss.blue.4" },

    "moss.error": { type: "variable", value: "moss.red.3" },
    "moss.error.background": { type: "variable", value: "moss.red.9" },

    "moss.success": { type: "variable", value: "moss.green.3" },
    "moss.success.background": { type: "variable", value: "moss.green.11" },

    "moss.background.disabled": { type: "variable", value: "moss.gray.12" },
    "moss.border.disabled": { type: "variable", value: "moss.gray.12" },
    "moss.foreground.disabled": { type: "variable", value: "moss.gray.8" },

    "moss.border": { type: "variable", value: "moss.gray.12" },

    "moss.primary.background": { type: "variable", value: "moss.gray.14" },
    "moss.primary.background.hover": { type: "variable", value: "moss.blue.13" },
    "moss.primary.foreground": { type: "variable", value: "moss.gray.1" },
    "moss.primary.descriptionForeground": { type: "variable", value: "moss.gray.7" },

    "moss.secondary.background": { type: "variable", value: "moss.gray.13" },
    "moss.secondary.background.hover": { type: "variable", value: "moss.gray.12" },
    "moss.secondary.background.active": { type: "variable", value: "moss.gray.11" },
    "moss.secondary.foreground": { type: "variable", value: "moss.gray.6" },

    // status bar
    "moss.statusBarItem.foreground": { type: "variable", value: "moss.gray.4" },
    "moss.statusBarItem.background.hover": { type: "variable", value: "moss.gray.12" },

    // activity bar

    "moss.activityBarItem.background": { type: "variable", value: "moss.gray.12" },
    "moss.activityBarItem.background.hover": { type: "variable", value: "moss.gray.10" },
    "moss.activityBarItem.foreground": { type: "variable", value: "moss.gray.6" },

    // toolbar
    "moss.toolbarItem.background": { type: "variable", value: "moss.gray.14" },
    "moss.toolbarItem.background.hover": { type: "variable", value: "moss.gray.12" },
    "moss.toolbarItem.foreground": { type: "variable", value: "moss.gray.6" },

    // list
    "moss.list.background": { type: "variable", value: "moss.gray.13" },
    "moss.list.background.hover": { type: "variable", value: "moss.gray.12" },
    "moss.list.background.active": { type: "variable", value: "moss.blue.11" },
    "moss.list.foreground": { type: "variable", value: "moss.gray.1" },
    "moss.list.descriptionForeground": { type: "variable", value: "moss.gray.7" },

    "moss.list.toolbarItem.background": { type: "variable", value: "transparent" },
    "moss.list.toolbarItem.background.hover": { type: "variable", value: "moss.gray.10" },

    //buttons
    "moss.button.default": { type: "variable", value: "moss.gray.13" },
    "moss.button.danger": { type: "variable", value: "moss.red.4" },

    "moss.button.outlined.background": { type: "variable", value: "moss.gray.14" },
    "moss.button.outlined.background.hover": { type: "variable", value: "moss.gray.12" },
    "moss.button.outlined.border": { type: "variable", value: "moss.gray.9" },
    "moss.button.outlined.border.hover": { type: "variable", value: "moss.gray.8" },
    "moss.button.outlined.foreground": { type: "variable", value: "moss.gray.1" },

    //input, textarea, select, checkbox, radio, toggle, IconLabelButton etc.
    "moss.controls.background": { type: "variable", value: "moss.gray.14" },
    "moss.controls.background.hover": { type: "variable", value: "moss.gray.12" },
    "moss.controls.background.contrast": { type: "variable", value: "moss.gray.13" },
    "moss.controls.border": { type: "variable", value: "moss.gray.10" },
    "moss.controls.foreground": { type: "variable", value: "moss.gray.1" },
    "moss.controls.placeholder": { type: "variable", value: "moss.gray.6" },
    "moss.controls.shortcut.background": { type: "variable", value: "moss.gray.12" },
    "moss.controls.shortcut.foreground": { type: "variable", value: "moss.gray.3" },

    "moss.workspaceMode.background": { type: "variable", value: "moss.gray.12" },
    "moss.workspaceMode.border": { type: "variable", value: "moss.gray.11" },
    "moss.workspaceMode.foreground": { type: "variable", value: "moss.gray.7" },
    "moss.workspaceMode.foreground.selected": { type: "variable", value: "moss.gray.1" },

    "moss.toggleButton.background": { type: "variable", value: "moss.gray.12" },
    "moss.toggleButton.border": { type: "variable", value: "moss.gray.11" },
    "moss.toggleButton.indicator": { type: "variable", value: "moss.gray.6" },
    "moss.toggleButton.indicator.checked": { type: "variable", value: "moss.gray.14" },
    "moss.toggleButton.thumb": { type: "variable", value: "moss.gray.14" },
    "moss.toggleButton.thumb.border": { type: "variable", value: "moss.gray.8" },

    "moss.notification.background": { type: "variable", value: "moss.gray.2" },
    "moss.notification.foreground": { type: "variable", value: "moss.gray.14" },
    "moss.notification.button.outline": { type: "variable", value: "moss.gray.7" },
    "moss.notification.button.hover": { type: "variable", value: "moss.gray.4" },
    "moss.notification.close": { type: "variable", value: "moss.gray.6" },

    "moss.link.foreground": { type: "variable", value: "moss.blue.8" },
    "moss.link.foreground.hover": { type: "variable", value: "moss.blue.10" },

    "moss.stepCard.background": { type: "variable", value: "moss.blue.12" },
    "moss.stepCard.foreground": { type: "variable", value: "moss.blue.4" },

    //resizable handle
    "separator.border": { type: "variable", value: "moss.border" },

    "moss.windowsCloseButton.button.icon": { type: "solid", value: "black" },
    "moss.windowsCloseButton.background": { type: "solid", value: rgba(196, 43, 28, 1) },
    "moss.windowControlsLinux.background": { type: "solid", value: "#e7e7e7" },
    "moss.windowControlsLinux.foreground": { type: "solid", value: rgba(61, 61, 61, 1) },
    "moss.windowControlsLinux.hoverBackground": { type: "solid", value: rgba(209, 209, 209, 1) },
    "moss.windowControlsLinux.activeBackground": { type: "solid", value: rgba(191, 191, 191, 1) },
  },
  boxShadows: {
    "moss.floating.box.shadow": "8px 8px 8px 0px rgba(83, 89, 93, 0.5)",
  },
};
