import { Theme } from "../theme";
import { rgba } from "../color";

export const defaultVSCodeTheme: Theme = {
  "identifier": "moss.sapic-theme.vscode",
  "displayName": "VS Code",
  "mode": "dark",
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
    "moss.primary": {
      type: "solid",
      value: "#0065ff",
    },
    "moss.error": {
      type: "solid",
      value: "#f48771",
    },
  },
  colors: {
    "moss.error.background": {
      type: "solid",
      value: "#f7d7d7",
    },
    "moss.success.background": {
      type: "solid",
      value: "#f2fcf3",
    },
    "moss.border.color": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.primary.background": {
      type: "solid",
      value: "#1e1e1e",
    },
    "moss.primary.background.hover": {
      type: "solid",
      value: "#d4e2ff",
    },
    "moss.primary.text": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.secondary.background": {
      type: "solid",
      value: "#333333",
    },
    "moss.secondary.background.hover": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.secondary.text": {
      type: "solid",
      value: "#cccccc",
    },
    "moss.info.background": {
      type: "solid",
      value: "#edf3ff",
    },
    "moss.info.background.hover": {
      type: "solid",
      value: "#d4e2ff",
    },
    "moss.info.text": {
      type: "variable",
      value: "moss.primary.text",
    },
    "moss.info.icon": {
      type: "variable",
      value: "moss.primary",
    },
    "moss.info.border": {
      type: "variable",
      value: "moss.primary",
    },
    "moss.divider.color": {
      type: "variable",
      value: "moss.border.color",
    },
    "moss.not.selected.item.color": {
      type: "variable",
      value: "moss.border.color",
    },
    "separator.border": {
      type: "variable",
      value: "moss.border.color",
    },
    "moss.sash.active.highlight": {
      type: "solid",
      value: "#047cd4",
    },
    "moss.icon.primary.background": {
      type: "solid",
      value: "transparent",
    },
    "moss.icon.primary.text": {
      type: "variable",
      value: "moss.gray.7",
    },
    "moss.icon.primary.background.hover": {
      type: "solid",
      value: rgba(0, 0, 0, 0.33),
    },
    "moss.icon.primary.background.active": {
      type: "solid",
      value: "#3574f0",
    },
    "moss.headBar.icon.primary.text": {
      type: "variable",
      value: "moss.gray.4",
    },
    "moss.headBar.primary.background": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.headBar.primary.background.hover": {
      type: "variable",
      value: "moss.gray.11",
    },
    "moss.headBar.border.color": {
      type: "variable",
      value: "moss.gray.11",
    },
    "moss.statusBar.icon.primary.text": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.statusBar.icon.secondary.text": {
      type: "variable",
      value: "moss.gray.4",
    },
    "moss.statusBar.icon.background.hover": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.button.icon.color": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.windowsCloseButton.background": {
      type: "solid",
      value: rgba(196, 43, 28, 1),
    },
    "moss.windowControlsLinux.background": {
      type: "solid",
      value: "#333333",
    },
    "moss.windowControlsLinux.text": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.windowControlsLinux.hoverBackground": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.windowControlsLinux.activeBackground": {
      type: "solid",
      value: "#505050",
    },
    "moss.paneview.active.outline.color": {
      type: "variable",
      value: "moss.primary",
    },
    "moss.drag.over.background.color": {
      type: "solid",
      value: rgba(0, 122, 204, 0.2),
    },
    "moss.drag.over.border.color": {
      type: "variable",
      value: "moss.primary.background",
    },
    "moss.group.view.background.color": {
      type: "variable",
      value: "moss.primary.background",
    },
    "moss.tabs.and.actions.container.background.color": {
      type: "variable",
      value: "moss.primary.background",
    },
    "moss.activegroup.visiblepanel.tab.background.color": {
      type: "variable",
      value: "moss.primary.background.hover",
    },
    "moss.inactivegroup.visiblepanel.tab.background.color": {
      type: "variable",
      value: "moss.primary.background",
    },
    "moss.activegroup.visiblepanel.tab.color": {
      type: "variable",
      value: "moss.primary.text",
    },
    "moss.inactivegroup.visiblepanel.tab.color": {
      type: "variable",
      value: "moss.secondary.text",
    },
    "moss.activegroup.visiblepanel.tab.border.color": {
      type: "variable",
      value: "moss.primary",
    },
    "moss.activegroup.hiddenpanel.tab.border.color": {
      type: "solid",
      value: "transparent",
    },
    "moss.inactivegroup.visiblepanel.tab.border.color": {
      type: "variable",
      value: "moss.gray.8",
    },
    "moss.activegroup.hiddenpanel.tab.background.color": {
      type: "variable",
      value: "moss.primary.background",
    },
    "moss.inactivegroup.hiddenpanel.tab.background.color": {
      type: "variable",
      value: "moss.primary.background",
    },
    "moss.activegroup.hiddenpanel.tab.color": {
      type: "variable",
      value: "moss.secondary.text",
    },
    "moss.inactivegroup.hiddenpanel.tab.color": {
      type: "variable",
      value: "moss.secondary.text",
    },
    "moss.tab.divider.color": {
      type: "variable",
      value: "moss.border.color",
    },
    "moss.separator.border": {
      type: "variable",
      value: "moss.border.color",
    },
    "moss.button.primary.solid.background": {
      type: "solid",
      value: "#007acc",
    },
    "moss.button.primary.solid.border": {
      type: "solid",
      value: "#007acc",
    },
    "moss.button.primary.solid.text": {
      type: "solid",
      value: "#ffffff",
    },
    "moss.button.primary.outlined.background": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.primary.outlined.border": {
      type: "solid",
      value: "#007acc",
    },
    "moss.button.primary.outlined.text": {
      type: "solid",
      value: "#007acc",
    },
    "moss.button.primary.soft.background": {
      type: "solid",
      value: rgba(0, 122, 204, 0.1),
    },
    "moss.button.primary.soft.border": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.primary.soft.text": {
      type: "solid",
      value: "#007acc",
    },
    "moss.button.primary.ghost.background": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.primary.ghost.border": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.primary.ghost.text": {
      type: "solid",
      value: "#007acc",
    },
    "moss.button.danger.solid.background": {
      type: "solid",
      value: "#f48771",
    },
    "moss.button.danger.solid.border": {
      type: "solid",
      value: "#f48771",
    },
    "moss.button.danger.solid.text": {
      type: "solid",
      value: "#ffffff",
    },
    "moss.button.danger.outlined.background": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.danger.outlined.border": {
      type: "solid",
      value: "#f48771",
    },
    "moss.button.danger.outlined.text": {
      type: "solid",
      value: "#f48771",
    },
    "moss.button.danger.soft.background": {
      type: "solid",
      value: rgba(244, 135, 113, 0.1),
    },
    "moss.button.danger.soft.border": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.danger.soft.text": {
      type: "solid",
      value: "#f48771",
    },
    "moss.button.danger.ghost.background": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.danger.ghost.border": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.danger.ghost.text": {
      type: "solid",
      value: "#f48771",
    },
    "moss.button.neutral.solid.background": {
      type: "solid",
      value: "#424242",
    },
    "moss.button.neutral.solid.border": {
      type: "solid",
      value: "#424242",
    },
    "moss.button.neutral.solid.text": {
      type: "solid",
      value: "#ffffff",
    },
    "moss.button.neutral.outlined.background": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.neutral.outlined.border": {
      type: "solid",
      value: "#424242",
    },
    "moss.button.neutral.outlined.text": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.button.neutral.soft.background": {
      type: "solid",
      value: rgba(66, 66, 66, 0.1),
    },
    "moss.button.neutral.soft.border": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.neutral.soft.text": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.button.neutral.ghost.background": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.neutral.ghost.border": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.neutral.ghost.text": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.controls.plain.bg": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.controls.plain.border": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.controls.plain.text": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.controls.outlined.bg": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.controls.outlined.border": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.controls.outlined.text": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.controls.placeholder": {
      type: "solid",
      value: "#969696",
    },
    "moss.select.bg.outlined": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.select.border.outlined": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.select.text.outlined": {
      type: "solid",
      value: "#d4d4d4",
    },
    "moss.select.hover.bg": {
      type: "solid",
      value: "#505050",
    },
    "moss.input.bg.plain": {
      type: "solid",
      value: "#3c3c3c",
    },
    "moss.input.bg.outlined": {
      type: "solid",
      value: "#3c3c3c",
    },
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
