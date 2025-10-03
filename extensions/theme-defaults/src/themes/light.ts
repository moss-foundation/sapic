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
    "moss.primary": {
      type: "variable",
      value: "moss.blue.4",
    },
    "moss.error": {
      type: "variable",
      value: "moss.red.3",
    },
    "moss.error.background": {
      type: "variable",
      value: "moss.red.9",
    },
    "moss.success": {
      type: "variable",
      value: "moss.green.3",
    },
    "moss.success.background": {
      type: "variable",
      value: "moss.green.11",
    },
    "moss.border.color": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.primary.background": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.primary.background.hover": {
      type: "variable",
      value: "moss.blue.13",
    },
    "moss.primary.text": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.secondary.background": {
      type: "variable",
      value: "moss.gray.13",
    },
    "moss.secondary.background.hover": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.secondary.background.active": {
      type: "variable",
      value: "moss.gray.11",
    },
    "moss.secondary.text": {
      type: "variable",
      value: "moss.gray.6",
    },
    "moss.info.background": {
      type: "variable",
      value: "moss.blue.12",
    },
    "moss.info.background.hover": {
      type: "variable",
      value: "moss.blue.11",
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
      value: "moss.gray.11",
    },
    "moss.not.selected.item.color": {
      type: "variable",
      value: "moss.gray.7",
    },
    "moss.shortcut.text": {
      type: "variable",
      value: "moss.gray.7",
    },
    "moss.tree.entries.counter": {
      type: "variable",
      value: "moss.gray.7",
    },
    "focus.border": {
      type: "variable",
      value: "moss.primary",
    },
    "separator.border": {
      type: "variable",
      value: "moss.border.color",
    },
    "moss.icon.primary.background": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.icon.primary.text": {
      type: "variable",
      value: "moss.gray.6",
    },
    "moss.icon.primary.background.hover": {
      type: "variable",
      value: "moss.gray.10",
    },
    "moss.icon.primary.background.active": {
      type: "variable",
      value: "moss.blue.11",
    },
    "moss.icon.secondary.background": {
      type: "solid",
      value: "transparent",
    },
    "moss.icon.secondary.text": {
      type: "variable",
      value: "moss.gray.6",
    },
    "moss.icon.secondary.background.hover": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.icon.secondary.background.active": {
      type: "variable",
      value: "moss.gray.11",
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
    "moss.statusBar.background": {
      type: "variable",
      value: "moss.gray.2",
    },
    "moss.button.icon.color": {
      type: "solid",
      value: "black",
    },
    "moss.windowsCloseButton.background": {
      type: "solid",
      value: rgba(196, 43, 28, 1),
    },
    "moss.windowControlsLinux.background": {
      type: "solid",
      value: "#e7e7e7",
    },
    "moss.windowControlsLinux.text": {
      type: "solid",
      value: rgba(61, 61, 61, 1),
    },
    "moss.windowControlsLinux.hoverBackground": {
      type: "solid",
      value: rgba(209, 209, 209, 1),
    },
    "moss.windowControlsLinux.activeBackground": {
      type: "solid",
      value: rgba(191, 191, 191, 1),
    },
    "moss.sash.active.highlight": {
      type: "solid",
      value: "#047cd4",
    },
    "moss.paneview.active.outline.color": {
      type: "variable",
      value: "moss.primary",
    },
    "moss.drag.over.background.color": {
      type: "variable",
      value: "moss.blue.11",
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
      type: "variable",
      value: "moss.blue.4",
    },
    "moss.button.primary.solid.background.hover": {
      type: "variable",
      value: "moss.blue.3",
    },
    "moss.button.primary.solid.background.active": {
      type: "variable",
      value: "moss.blue.2",
    },
    "moss.button.primary.solid.border": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.primary.solid.border.hover": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.primary.solid.border.active": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.primary.solid.text": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.button.primary.outlined.background": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.button.primary.outlined.background.contrast": {
      type: "variable",
      value: "moss.gray.13",
    },
    "moss.button.primary.outlined.background.hover": {
      type: "variable",
      value: "moss.blue.12",
    },
    "moss.button.primary.outlined.background.active": {
      type: "variable",
      value: "moss.blue.11",
    },
    "moss.button.primary.outlined.border": {
      type: "variable",
      value: "moss.blue.4",
    },
    "moss.button.primary.outlined.border.hover": {
      type: "variable",
      value: "moss.blue.5",
    },
    "moss.button.primary.outlined.border.active": {
      type: "variable",
      value: "moss.blue.6",
    },
    "moss.button.primary.outlined.text": {
      type: "variable",
      value: "moss.blue.4",
    },
    "moss.button.neutral.solid.background": {
      type: "variable",
      value: "moss.gray.4",
    },
    "moss.button.neutral.solid.background.hover": {
      type: "variable",
      value: "moss.gray.3",
    },
    "moss.button.neutral.solid.background.active": {
      type: "variable",
      value: "moss.gray.2",
    },
    "moss.button.neutral.solid.border": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.neutral.solid.border.hover": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.neutral.solid.border.active": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.neutral.solid.text": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.button.neutral.outlined.background": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.button.neutral.outlined.background.hover": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.button.neutral.outlined.background.active": {
      type: "variable",
      value: "moss.gray.13",
    },
    "moss.button.neutral.outlined.border": {
      type: "variable",
      value: "moss.gray.9",
    },
    "moss.button.neutral.outlined.border.hover": {
      type: "variable",
      value: "moss.gray.8",
    },
    "moss.button.neutral.outlined.border.active": {
      type: "variable",
      value: "moss.gray.7",
    },
    "moss.button.neutral.outlined.text": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.button.background.disabled": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.button.background.disabled.hover": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.button.background.disabled.active": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.button.border.disabled": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.border.disabled.hover": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.border.disabled.active": {
      type: "solid",
      value: "transparent",
    },
    "moss.button.text.disabled": {
      type: "variable",
      value: "moss.gray.8",
    },
    "moss.button.background.danger": {
      type: "variable",
      value: "moss.red.4",
    },
    "moss.button.background.danger.hover": {
      type: "variable",
      value: "moss.red.2",
    },
    "moss.button.text.danger": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.controls.plain.bg": {
      type: "solid",
      value: "transparent",
    },
    "moss.controls.plain.border": {
      type: "solid",
      value: "none",
    },
    "moss.controls.plain.text": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.controls.outlined.bg": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.controls.outlined.border": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.controls.outlined.text": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.controls.placeholder": {
      type: "variable",
      value: "moss.gray.6",
    },
    "moss.select.bg.outlined": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.select.border.outlined": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.select.text.outlined": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.select.hover.bg": {
      type: "variable",
      value: "moss.gray.13",
    },
    "moss.select.item.bg.outlined.hover": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.select.item.bg.outlined.selected": {
      type: "variable",
      value: "moss.blue.11",
    },
    "moss.select.disabled.bg": {
      type: "variable",
      value: "moss.gray.10",
    },
    "moss.input.bg.plain": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.input.bg.outlined": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.input.border": {
      type: "variable",
      value: "moss.gray.9",
    },
    "moss.stepCard.text": {
      type: "variable",
      value: "moss.blue.2",
    },
    "moss.stepCard.bg": {
      type: "variable",
      value: "moss.blue.11",
    },
    "moss.templating.input.text": {
      type: "variable",
      value: "moss.blue.3",
    },
    "moss.templating.input.border": {
      type: "variable",
      value: "moss.blue.10",
    },
    "moss.templating.input.bg": {
      type: "variable",
      value: "moss.blue.12",
    },
    "moss.checkbox.border": {
      type: "variable",
      value: "moss.gray.8",
    },
    "moss.checkbox.border.disabled": {
      type: "variable",
      value: "moss.gray.10",
    },
    "moss.checkbox.bg.disabled": {
      type: "variable",
      value: "moss.gray.8",
    },
    "moss.radio.bg": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.radio.border": {
      type: "variable",
      value: "moss.gray.8",
    },
    "moss.radio.bg.disabled": {
      type: "variable",
      value: "moss.gray.10",
    },
    "moss.radio.border.disabled": {
      type: "variable",
      value: "moss.gray.10",
    },
    "moss.table.header.bg": {
      type: "variable",
      value: "moss.gray.13",
    },
    "moss.table.cell.bg": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.table.add.form.text": {
      type: "variable",
      value: "moss.gray.9",
    },
    "moss.table.add.row.form.text": {
      type: "variable",
      value: "moss.gray.9",
    },
    "moss.drag.handle.bg": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.tab.active.border.color": {
      type: "variable",
      value: "moss.blue.4",
    },
    "moss.auth.indicator.color": {
      type: "variable",
      value: "moss.green.4",
    },
    "moss.tab.badge.color": {
      type: "variable",
      value: "moss.blue.4",
    },
    "moss.tab.badge.text": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.requestpage.border.color": {
      type: "variable",
      value: "moss.gray.11",
    },
    "moss.requestpage.text": {
      type: "variable",
      value: "moss.orange.4",
    },
    "moss.requestpage.string.color": {
      type: "variable",
      value: "moss.green.3",
    },
    "moss.requestpage.number.color": {
      type: "variable",
      value: "moss.purple.3",
    },
    "moss.requestpage.bool.color": {
      type: "variable",
      value: "moss.teal.3",
    },
    "moss.requestpage.icon.color": {
      type: "variable",
      value: "moss.gray.7",
    },
    "moss.requestpage.text.disabled": {
      type: "variable",
      value: "moss.gray.8",
    },
    "moss.requestpage.placeholder.color": {
      type: "variable",
      value: "moss.gray.9",
    },
    "moss.requestpage.header.color": {
      type: "variable",
      value: "moss.gray.3",
    },
    "moss.display.mode.border": {
      type: "variable",
      value: "moss.gray.11",
    },
    "moss.display.mode.bg": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.display.mode.text": {
      type: "variable",
      value: "moss.gray.7",
    },
    "moss.display.mode.text.selected": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.textarea.bg": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.textarea.border": {
      type: "variable",
      value: "moss.gray.9",
    },
    "moss.textarea.text": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.textarea.placeholder": {
      type: "variable",
      value: "moss.gray.6",
    },
    "moss.mossToggle.bg": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.mossToggle.border": {
      type: "variable",
      value: "moss.gray.11",
    },
    "moss.mossToggle.indicator": {
      type: "variable",
      value: "moss.gray.6",
    },
    "moss.mossToggle.indicator.checked": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.mossToggle.thumb": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.mossToggle.thumb.border": {
      type: "variable",
      value: "moss.gray.8",
    },
    "moss.mossSelect.bg": {
      type: "variable",
      value: "moss.gray.13",
    },
    "moss.mossSelect.border": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.mossSelect.text": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.mossSelect.hover.bg": {
      type: "variable",
      value: "moss.gray.13",
    },
    "moss.mossSelect.item.bg.hover": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.mossSelect.item.bg.selected": {
      type: "variable",
      value: "moss.blue.11",
    },
    "moss.mossSelect.disabled.bg": {
      type: "variable",
      value: "moss.gray.10",
    },
    "moss.mossDropdown.bg": {
      type: "variable",
      value: "moss.gray.13",
    },
    "moss.mossDropdown.border": {
      type: "variable",
      value: "moss.gray.11",
    },
    "moss.mossDropdown.text": {
      type: "variable",
      value: "moss.gray.1",
    },
    "moss.mossDropdown.hover.bg": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.mossDropdown.item.bg.hover": {
      type: "variable",
      value: "moss.gray.12",
    },
    "moss.notification.bg": {
      type: "variable",
      value: "moss.gray.2",
    },
    "moss.notification.text": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.notification.button.outline": {
      type: "variable",
      value: "moss.gray.7",
    },
    "moss.notification.button.hover": {
      type: "variable",
      value: "moss.gray.4",
    },
    "moss.notification.close.color": {
      type: "variable",
      value: "moss.gray.6",
    },
    "moss.link.text": {
      type: "variable",
      value: "moss.blue.8",
    },
    "moss.link.hover": {
      type: "variable",
      value: "moss.blue.10",
    },
  },
  boxShadows: {
    "moss.floating.box.shadow": "8px 8px 8px 0px rgba(83, 89, 93, 0.5)",
  },
};
