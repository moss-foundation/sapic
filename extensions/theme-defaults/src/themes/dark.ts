import { rgba } from "../color";
import { Theme } from "../theme";

export const defaultDarkTheme: Theme = {
  identifier: "moss.sapic-theme.darkDefault",
  displayName: "Dark Default",
  mode: "dark",
  palette: {
    "moss.gray.1": {
      type: "solid",
      value: "#1e1f22",
    },
    "moss.gray.2": {
      type: "solid",
      value: "#2b2d30",
    },
    "moss.gray.3": {
      type: "solid",
      value: "#393b40",
    },
    "moss.gray.4": {
      type: "solid",
      value: "#43454a",
    },
    "moss.gray.5": {
      type: "solid",
      value: "#4e5157",
    },
    "moss.gray.6": {
      type: "solid",
      value: "#5a5d63",
    },
    "moss.gray.7": {
      type: "solid",
      value: "#6f737a",
    },
    "moss.gray.8": {
      type: "solid",
      value: "#868a91",
    },
    "moss.gray.9": {
      type: "solid",
      value: "#9da0a8",
    },
    "moss.gray.10": {
      type: "solid",
      value: "#b4b8bf",
    },
    "moss.gray.11": {
      type: "solid",
      value: "#ced0d6",
    },
    "moss.gray.12": {
      type: "solid",
      value: "#dfe1e5",
    },
    "moss.gray.13": {
      type: "solid",
      value: "#f0f1f2",
    },
    "moss.gray.14": {
      type: "solid",
      value: "#ffffff",
    },
    "moss.blue.1": {
      type: "solid",
      value: "#25324d",
    },
    "moss.blue.2": {
      type: "solid",
      value: "#2e436e",
    },
    "moss.blue.3": {
      type: "solid",
      value: "#35538f",
    },
    "moss.blue.4": {
      type: "solid",
      value: "#375fad",
    },
    "moss.blue.5": {
      type: "solid",
      value: "#366acf",
    },
    "moss.blue.6": {
      type: "solid",
      value: "#3574f0",
    },
    "moss.blue.7": {
      type: "solid",
      value: "#467ff2",
    },
    "moss.blue.8": {
      type: "solid",
      value: "#548af7",
    },
    "moss.blue.9": {
      type: "solid",
      value: "#6b9bfa",
    },
    "moss.blue.10": {
      type: "solid",
      value: "#83acfc",
    },
    "moss.blue.11": {
      type: "solid",
      value: "#99bbff",
    },
    "moss.green.1": {
      type: "solid",
      value: "#253627",
    },
    "moss.green.2": {
      type: "solid",
      value: "#375239",
    },
    "moss.green.3": {
      type: "solid",
      value: "#436946",
    },
    "moss.green.4": {
      type: "solid",
      value: "#4e8052",
    },
    "moss.green.5": {
      type: "solid",
      value: "#57965c",
    },
    "moss.green.6": {
      type: "solid",
      value: "#5fad65",
    },
    "moss.green.7": {
      type: "solid",
      value: "#73bd79",
    },
    "moss.green.8": {
      type: "solid",
      value: "#89cc8e",
    },
    "moss.green.9": {
      type: "solid",
      value: "#a0dba5",
    },
    "moss.green.10": {
      type: "solid",
      value: "#b9ebbd",
    },
    "moss.green.11": {
      type: "solid",
      value: "#d4fad7",
    },
    "moss.yellow.1": {
      type: "solid",
      value: "#3d3223",
    },
    "moss.yellow.2": {
      type: "solid",
      value: "#5e4d33",
    },
    "moss.yellow.3": {
      type: "solid",
      value: "#826a41",
    },
    "moss.yellow.4": {
      type: "solid",
      value: "#9e814a",
    },
    "moss.yellow.5": {
      type: "solid",
      value: "#ba9752",
    },
    "moss.yellow.6": {
      type: "solid",
      value: "#d6ae58",
    },
    "moss.yellow.7": {
      type: "solid",
      value: "#f2c55c",
    },
    "moss.yellow.8": {
      type: "solid",
      value: "#f5d273",
    },
    "moss.yellow.9": {
      type: "solid",
      value: "#f7de8b",
    },
    "moss.yellow.10": {
      type: "solid",
      value: "#fceba4",
    },
    "moss.yellow.11": {
      type: "solid",
      value: "#fff6bd",
    },
    "moss.red.1": {
      type: "solid",
      value: "#402929",
    },
    "moss.red.2": {
      type: "solid",
      value: "#5e3838",
    },
    "moss.red.3": {
      type: "solid",
      value: "#7a4343",
    },
    "moss.red.4": {
      type: "solid",
      value: "#9c4e4e",
    },
    "moss.red.5": {
      type: "solid",
      value: "#bd5757",
    },
    "moss.red.6": {
      type: "solid",
      value: "#db5c5c",
    },
    "moss.red.7": {
      type: "solid",
      value: "#e37774",
    },
    "moss.red.8": {
      type: "solid",
      value: "#eb938d",
    },
    "moss.red.9": {
      type: "solid",
      value: "#f2b1aa",
    },
    "moss.red.10": {
      type: "solid",
      value: "#f7ccc6",
    },
    "moss.red.11": {
      type: "solid",
      value: "#fae3de",
    },
    "moss.orange.1": {
      type: "solid",
      value: "#45322b",
    },
    "moss.orange.2": {
      type: "solid",
      value: "#614438",
    },
    "moss.orange.3": {
      type: "solid",
      value: "#825845",
    },
    "moss.orange.4": {
      type: "solid",
      value: "#a36b4e",
    },
    "moss.orange.5": {
      type: "solid",
      value: "#c77d55",
    },
    "moss.orange.6": {
      type: "solid",
      value: "#e08855",
    },
    "moss.orange.7": {
      type: "solid",
      value: "#e5986c",
    },
    "moss.orange.8": {
      type: "solid",
      value: "#f0ac81",
    },
    "moss.orange.9": {
      type: "solid",
      value: "#f5bd98",
    },
    "moss.orange.10": {
      type: "solid",
      value: "#faceaf",
    },
    "moss.orange.11": {
      type: "solid",
      value: "#ffdfc7",
    },
    "moss.purple.1": {
      type: "solid",
      value: "#2f2936",
    },
    "moss.purple.2": {
      type: "solid",
      value: "#433358",
    },
    "moss.purple.3": {
      type: "solid",
      value: "#583d7a",
    },
    "moss.purple.4": {
      type: "solid",
      value: "#6c469c",
    },
    "moss.purple.5": {
      type: "solid",
      value: "#8150be",
    },
    "moss.purple.6": {
      type: "solid",
      value: "#955ae0",
    },
    "moss.purple.7": {
      type: "solid",
      value: "#a571e6",
    },
    "moss.purple.8": {
      type: "solid",
      value: "#b589ec",
    },
    "moss.purple.9": {
      type: "solid",
      value: "#c4a0f3",
    },
    "moss.purple.10": {
      type: "solid",
      value: "#d4b8f9",
    },
    "moss.purple.11": {
      type: "solid",
      value: "#e4ceff",
    },
    "moss.teal.1": {
      type: "solid",
      value: "#1d3838",
    },
    "moss.teal.2": {
      type: "solid",
      value: "#1e4d4a",
    },
    "moss.teal.3": {
      type: "solid",
      value: "#20635d",
    },
    "moss.teal.4": {
      type: "solid",
      value: "#21786f",
    },
    "moss.teal.5": {
      type: "solid",
      value: "#238e82",
    },
    "moss.teal.6": {
      type: "solid",
      value: "#24a394",
    },
    "moss.teal.7": {
      type: "solid",
      value: "#42b1a4",
    },
    "moss.teal.8": {
      type: "solid",
      value: "#60c0b5",
    },
    "moss.teal.9": {
      type: "solid",
      value: "#7dcec5",
    },
    "moss.teal.10": {
      type: "solid",
      value: "#9bddd6",
    },
    "moss.teal.11": {
      type: "solid",
      value: "#b9ebe6",
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

    "moss.border": { type: "variable", value: "moss.gray.1" },

    "moss.primary.background": { type: "variable", value: "moss.gray.1" },
    "moss.primary.background.hover": { type: "variable", value: "moss.gray.2" },
    "moss.primary.foreground": { type: "variable", value: "moss.gray.14" },
    "moss.primary.descriptionForeground": { type: "variable", value: "moss.gray.7" },

    "moss.secondary.background": { type: "variable", value: "moss.gray.2" },
    "moss.secondary.background.hover": { type: "variable", value: "moss.gray.3" },
    "moss.secondary.background.active": { type: "variable", value: "moss.gray.4" },
    "moss.secondary.foreground": { type: "variable", value: "moss.gray.9" },

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
    "moss.activityBarItem.foreground": { type: "variable", value: "moss.gray.9" },

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
    "moss.floating.box.shadow": "8px 8px 8px 0px rgba(83, 89, 93, 0.5)",
    "moss.button.primary.solid.boxShadow": "rgba(255, 255, 255, 0.1) 0px 2px 4px 0px inset",
    "moss.button.primary.outlined.boxShadow":
      "rgba(255, 255, 255, 0) 0px 1px 0px 0px inset, rgba(59, 130, 246, 0.3) 0px 0px 0px 1px,\r\n    rgba(0, 0, 0, 0.1) 0px 1px 2px 0px",
    "moss.button.danger.solid.boxShadow":
      "rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(255, 255, 255, 0.1) 0px 2px 4px 0px inset",
    "moss.button.danger.outlined.boxShadow":
      "rgba(255, 255, 255, 0) 0px 1px 0px 0px inset, rgba(239, 68, 68, 0.3) 0px 0px 0px 1px,\r\n    rgba(0, 0, 0, 0.1) 0px 1px 2px 0px",
    "moss.button.primary.soft.boxShadow": "null",
    "moss.button.primary.ghost.boxShadow": "null",
    "moss.button.danger.soft.boxShadow": "null",
    "moss.button.danger.ghost.boxShadow": "null",
    "moss.button.neutral.solid.boxShadow": "rgba(255, 255, 255, 0.1) 0px 2px 4px 0px inset",
    "moss.button.neutral.outlined.boxShadow":
      "rgba(255, 255, 255, 0) 0px 1px 0px 0px inset, rgba(113, 113, 122, 0.3) 0px 0px 0px 1px,\r\n    rgba(0, 0, 0, 0.1) 0px 1px 2px 0px",
    "moss.button.neutral.soft.boxShadow": "null",
    "moss.button.neutral.ghost.boxShadow": "null",
  },
};
