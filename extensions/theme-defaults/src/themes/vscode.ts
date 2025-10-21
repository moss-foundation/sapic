import { rgba } from "../color";
import { Theme } from "../theme";

export const defaultVSCodeTheme: Theme = {
  "identifier": "moss.sapic-theme.vscode",
  "displayName": "VS Code",
  "mode": "dark",
  "palette": {
    "moss.gray.1": {
      "type": "solid",
      "value": "#000000",
    },
    "moss.gray.2": {
      "type": "solid",
      "value": "#27282e",
    },
    "moss.gray.3": {
      "type": "solid",
      "value": "#383a42",
    },
    "moss.gray.4": {
      "type": "solid",
      "value": "#494b57",
    },
    "moss.gray.5": {
      "type": "solid",
      "value": "#5a5d6b",
    },
    "moss.gray.6": {
      "type": "solid",
      "value": "#6c707e",
    },
    "moss.gray.7": {
      "type": "solid",
      "value": "#818594",
    },
    "moss.gray.8": {
      "type": "solid",
      "value": "#a8adbd",
    },
    "moss.gray.9": {
      "type": "solid",
      "value": "#c9ccd6",
    },
    "moss.gray.10": {
      "type": "solid",
      "value": "#d3d5db",
    },
    "moss.gray.11": {
      "type": "solid",
      "value": "#dfe1e5",
    },
    "moss.gray.12": {
      "type": "solid",
      "value": "#ebecf0",
    },
    "moss.gray.13": {
      "type": "solid",
      "value": "#f7f8fa",
    },
    "moss.gray.14": {
      "type": "solid",
      "value": "#ffffff",
    },
    "moss.blue.1": {
      "type": "solid",
      "value": "#2e55a3",
    },
    "moss.blue.2": {
      "type": "solid",
      "value": "#315fbd",
    },
    "moss.blue.3": {
      "type": "solid",
      "value": "#3369d6",
    },
    "moss.blue.4": {
      "type": "solid",
      "value": "#3574f0",
    },
    "moss.blue.5": {
      "type": "solid",
      "value": "#4682fa",
    },
    "moss.blue.6": {
      "type": "solid",
      "value": "#588cf3",
    },
    "moss.blue.7": {
      "type": "solid",
      "value": "#709cf5",
    },
    "moss.blue.8": {
      "type": "solid",
      "value": "#88adf7",
    },
    "moss.blue.9": {
      "type": "solid",
      "value": "#a0bdf8",
    },
    "moss.blue.10": {
      "type": "solid",
      "value": "#c2d6fc",
    },
    "moss.blue.11": {
      "type": "solid",
      "value": "#d4e2ff",
    },
    "moss.blue.12": {
      "type": "solid",
      "value": "#edf3ff",
    },
    "moss.blue.13": {
      "type": "solid",
      "value": "#f5f8fe",
    },
    "moss.green.1": {
      "type": "solid",
      "value": "#1e6b33",
    },
    "moss.green.2": {
      "type": "solid",
      "value": "#1f7536",
    },
    "moss.green.3": {
      "type": "solid",
      "value": "#1f8039",
    },
    "moss.green.4": {
      "type": "solid",
      "value": "#208a3c",
    },
    "moss.green.5": {
      "type": "solid",
      "value": "#369650",
    },
    "moss.green.6": {
      "type": "solid",
      "value": "#55a76a",
    },
    "moss.green.7": {
      "type": "solid",
      "value": "#89c398",
    },
    "moss.green.8": {
      "type": "solid",
      "value": "#afdbb8",
    },
    "moss.green.9": {
      "type": "solid",
      "value": "#c5e5cc",
    },
    "moss.green.10": {
      "type": "solid",
      "value": "#e6f7e9",
    },
    "moss.green.11": {
      "type": "solid",
      "value": "#f2fcf3",
    },
    "moss.red.1": {
      "type": "solid",
      "value": "#ad2b38",
    },
    "moss.red.2": {
      "type": "solid",
      "value": "#bc303e",
    },
    "moss.red.3": {
      "type": "solid",
      "value": "#cc3645",
    },
    "moss.red.4": {
      "type": "solid",
      "value": "#db3b4b",
    },
    "moss.red.5": {
      "type": "solid",
      "value": "#e55765",
    },
    "moss.red.6": {
      "type": "solid",
      "value": "#e46a76",
    },
    "moss.red.7": {
      "type": "solid",
      "value": "#ed99a1",
    },
    "moss.red.8": {
      "type": "solid",
      "value": "#f2b6bb",
    },
    "moss.red.9": {
      "type": "solid",
      "value": "#fad4d8",
    },
    "moss.red.10": {
      "type": "solid",
      "value": "#fff2f3",
    },
    "moss.red.11": {
      "type": "solid",
      "value": "#fff7f7",
    },
    "moss.primary": {
      "type": "solid",
      "value": "#0065ff",
    },
    "moss.error": {
      "type": "solid",
      "value": "#f48771",
    },
  },
  colors: {
    //general
    "moss.primary": { type: "variable", value: "moss.blue.6" },

    "moss.error": { type: "variable", value: "moss.red.6" },
    "moss.error.background": { type: "variable", value: "moss.red.9" },

    "moss.success": { type: "variable", value: "moss.green.6" },
    "moss.success.background": { type: "variable", value: "moss.green.11" },

    "moss.background.disabled": { type: "variable", value: "moss.gray.3" },
    "moss.border.disabled": { type: "variable", value: "moss.gray.3" },
    "moss.foreground.disabled": { type: "variable", value: "moss.gray.8" },

    "moss.border": { type: "variable", value: "moss.gray.5" },

    "moss.primary.background": { type: "variable", value: "moss.gray.1" },
    "moss.primary.background.hover": { type: "variable", value: "moss.gray.2" },
    "moss.primary.foreground": { type: "variable", value: "moss.gray.14" },
    "moss.primary.descriptionForeground": { type: "variable", value: "moss.gray.7" },

    "moss.secondary.background": { type: "variable", value: "moss.gray.2" },
    "moss.secondary.background.hover": { type: "variable", value: "moss.gray.3" },
    "moss.secondary.background.active": { type: "variable", value: "moss.gray.4" },
    "moss.secondary.foreground": { type: "variable", value: "moss.gray.9" },

    //TODO distribute these
    "moss.shortcut.foreground": { type: "variable", value: "moss.gray.7" },

    // head bar
    "moss.headBar.background": { type: "variable", value: "moss.gray.2" },
    "moss.headBar.border": { type: "variable", value: "moss.gray.4" },

    //Sidebar
    "moss.sidebar.background": { type: "variable", value: "moss.gray.2" },
    "moss.sidebar.foreground": { type: "variable", value: "moss.gray.9" },

    // status bar
    "moss.statusBar.background": { type: "variable", value: "moss.gray.2" },

    "moss.statusBarItem.foreground": { type: "variable", value: "moss.gray.9" },
    "moss.statusBarItem.background.hover": { type: "variable", value: "moss.gray.3" },

    // activity bar
    "moss.activityBar.background": { type: "variable", value: "moss.gray.2" },

    "moss.activityBarItem.background": { type: "variable", value: "moss.gray.3" },
    "moss.activityBarItem.background.hover": { type: "variable", value: "moss.gray.4" },
    "moss.activityBarItem.background.active": { type: "variable", value: "moss.blue.11" }, //deprecated
    "moss.activityBarItem.foreground": { type: "variable", value: "moss.gray.9" },
    "moss.activityBarItem.foreground.active": { type: "variable", value: "moss.blue.6" }, //deprecated

    // toolbar
    "moss.toolbarItem.background": { type: "variable", value: "moss.gray.1" },
    "moss.toolbarItem.background.hover": { type: "variable", value: "moss.gray.3" },
    "moss.toolbarItem.foreground": { type: "variable", value: "moss.gray.9" },

    // list
    "moss.list.background": { type: "variable", value: "moss.gray.2" },
    "moss.list.background.hover": { type: "variable", value: "moss.gray.3" },
    "moss.list.background.active": { type: "variable", value: "moss.blue.11" },
    "moss.list.foreground": { type: "variable", value: "moss.gray.14" },
    "moss.list.descriptionForeground": { type: "variable", value: "moss.gray.7" },

    "moss.list.toolbarItem.background": { type: "variable", value: "transparent" },
    "moss.list.toolbarItem.background.hover": { type: "variable", value: "moss.gray.4" },

    //buttons
    "moss.button.primary.background": { type: "variable", value: "moss.blue.6" },
    "moss.button.primary.background.hover": { type: "variable", value: "moss.blue.5" },
    "moss.button.primary.foreground": { type: "variable", value: "moss.gray.14" },

    "moss.button.outlined.background": { type: "variable", value: "moss.gray.1" },
    "moss.button.outlined.background.hover": { type: "variable", value: "moss.gray.3" },
    "moss.button.outlined.border": { type: "variable", value: "moss.gray.6" },
    "moss.button.outlined.border.hover": { type: "variable", value: "moss.gray.7" },
    "moss.button.outlined.foreground": { type: "variable", value: "moss.gray.14" },

    "moss.button.danger.background": { type: "variable", value: "moss.red.6" },
    "moss.button.danger.background.hover": { type: "variable", value: "moss.red.5" },
    "moss.button.danger.foreground": { type: "variable", value: "moss.gray.14" },

    //input, textarea, select, checkbox, radio, toggle, IconLabelButton etc.
    "moss.controls.background": { type: "variable", value: "moss.gray.1" },
    "moss.controls.background.hover": { type: "variable", value: "moss.gray.3" },
    "moss.controls.background.contrast": { type: "variable", value: "moss.gray.2" },
    "moss.controls.border": { type: "variable", value: "moss.gray.5" },
    "moss.controls.foreground": { type: "variable", value: "moss.gray.14" },
    "moss.controls.placeholder": { type: "variable", value: "moss.gray.8" },

    "moss.workspaceMode.background": { type: "variable", value: "moss.gray.3" },
    "moss.workspaceMode.border": { type: "variable", value: "moss.gray.4" },
    "moss.workspaceMode.foreground": { type: "variable", value: "moss.gray.9" },
    "moss.workspaceMode.foreground.selected": { type: "variable", value: "moss.gray.14" },

    "moss.toggleButton.background": { type: "variable", value: "moss.gray.3" },
    "moss.toggleButton.border": { type: "variable", value: "moss.gray.4" },
    "moss.toggleButton.indicator": { type: "variable", value: "moss.gray.8" },
    "moss.toggleButton.indicator.checked": { type: "variable", value: "moss.gray.14" },
    "moss.toggleButton.thumb": { type: "variable", value: "moss.gray.14" },
    "moss.toggleButton.thumb.border": { type: "variable", value: "moss.gray.6" },

    "moss.notification.background": { type: "variable", value: "moss.gray.3" },
    "moss.notification.foreground": { type: "variable", value: "moss.gray.14" },
    "moss.notification.button.outline": { type: "variable", value: "moss.gray.7" },
    "moss.notification.button.hover": { type: "variable", value: "moss.gray.5" },
    "moss.notification.close": { type: "variable", value: "moss.gray.7" },

    "moss.link.foreground": { type: "variable", value: "moss.blue.9" },
    "moss.link.foreground.hover": { type: "variable", value: "moss.blue.11" },

    "moss.stepCard.background": { type: "variable", value: "moss.blue.1" },
    "moss.stepCard.foreground": { type: "variable", value: "moss.blue.6" },

    //resizable handle
    "separator.border": { type: "variable", value: "moss.border" },

    "moss.windowsCloseButton.button.icon": { type: "solid", value: "white" },
    "moss.windowsCloseButton.background": { type: "solid", value: rgba(196, 43, 28, 1) },
    "moss.windowControlsLinux.background": { type: "solid", value: "#2b2b2b" },
    "moss.windowControlsLinux.foreground": { type: "solid", value: rgba(255, 255, 255, 1) },
    "moss.windowControlsLinux.hoverBackground": { type: "solid", value: rgba(65, 65, 65, 1) },
    "moss.windowControlsLinux.activeBackground": { type: "solid", value: rgba(75, 75, 75, 1) },
  },
  boxShadows: {
    "moss.floating.box.shadow": "0 2px 8px rgba(0, 0, 0, 0.15)",
    "moss.button.primary.solid.boxShadow": "none",
    "moss.button.primary.outlined.boxShadow": "none",
    "moss.button.primary.soft.boxShadow": "none",
    "moss.button.primary.ghost.boxShadow": "none",
    "moss.button.danger.solid.boxShadow": "none",
    "moss.button.danger.outlined.boxShadow": "none",
    "moss.button.danger.soft.boxShadow": "none",
    "moss.button.danger.ghost.boxShadow": "none",
    "moss.button.neutral.solid.boxShadow": "none",
    "moss.button.neutral.outlined.boxShadow": "none",
    "moss.button.neutral.soft.boxShadow": "none",
    "moss.button.neutral.ghost.boxShadow": "none",
  },
};
