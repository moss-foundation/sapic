import { Theme } from "../theme";
import { linearGradient, rgb, rgba } from "../color";

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
    "moss.primary": {
      type: "solid",
      value: "#0065ff",
    },
    "moss.error": {
      type: "solid",
      value: "#fb2c36",
    },
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
      value: rgba(22, 24, 25, 1),
    },
    "moss.primary.background.hover": {
      type: "solid",
      value: "#d4e2ff",
    },
    "moss.primary.text": {
      type: "solid",
      value: "white",
    },
    "moss.secondary.background": {
      type: "solid",
      value: rgba(39, 39, 42, 1),
    },
    "moss.secondary.background.hover": {
      type: "solid",
      value: rgba(255, 255, 255, 0.1),
    },
    "moss.secondary.text": {
      type: "solid",
      value: "#a1a1aa",
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
      value: "white",
    },
    "moss.windowsCloseButton.background": {
      type: "solid",
      value: rgba(196, 43, 28, 1),
    },
    "moss.windowControlsLinux.background": {
      type: "solid",
      value: rgba(55, 55, 55, 1),
    },
    "moss.windowControlsLinux.text": {
      type: "solid",
      value: rgba(255, 255, 255, 1),
    },
    "moss.windowControlsLinux.hoverBackground": {
      type: "solid",
      value: rgba(65, 65, 65, 1),
    },
    "moss.windowControlsLinux.activeBackground": {
      type: "solid",
      value: rgba(75, 75, 75, 1),
    },
    "moss.paneview.active.outline.color": {
      type: "variable",
      value: "moss.primary",
    },
    "moss.drag.over.background.color": {
      type: "solid",
      value: rgba(83, 89, 93, 0.5),
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
    "moss.sash.active.highlight": {
      type: "solid",
      value: "#047cd4",
    },
    "moss.button.primary.solid.background": {
      type: "gradient",
      value: linearGradient(
        null,
        {
          color: rgb(59, 130, 246),
        },
        {
          color: rgb(37, 99, 235),
          percentage: 0,
        }
      ),
    },
    "moss.button.primary.solid.border": {
      type: "solid",
      value: rgba(255, 255, 255, 0.3),
    },
    "moss.button.primary.solid.text": {
      type: "solid",
      value: "white",
    },
    "moss.button.primary.outlined.background": {
      type: "solid",
      value: rgba(59, 130, 246, 0.05),
    },
    "moss.button.primary.outlined.border": {
      type: "solid",
      value: rgba(59, 130, 246, 0.05),
    },
    "moss.button.primary.outlined.text": {
      type: "solid",
      value: rgb(147, 197, 253),
    },

    "moss.button.primary.soft.background": {
      type: "solid",
      value: rgba(59, 130, 246, 0.1),
    },
    "moss.button.primary.soft.border": {
      type: "solid",
      value: "null",
    },
    "moss.button.primary.soft.text": {
      type: "solid",
      value: rgb(147, 197, 253),
    },
    "moss.button.primary.ghost.background": {
      type: "solid",
      value: rgba(59, 130, 246, 0.1),
    },
    "moss.button.primary.ghost.border": {
      type: "solid",
      value: "null",
    },
    "moss.button.primary.ghost.text": {
      type: "solid",
      value: rgb(147, 197, 253),
    },
    "moss.button.danger.solid.background": {
      type: "gradient",
      value: linearGradient(null, { color: rgb(239, 68, 68) }, { color: rgb(220, 38, 38), percentage: 0 }),
    },
    "moss.button.danger.solid.border": {
      type: "solid",
      value: rgb(241, 95, 95),
    },
    "moss.button.danger.solid.text": {
      type: "solid",
      value: "white",
    },

    "moss.button.danger.outlined.background": {
      type: "solid",
      value: rgba(239, 68, 68, 0.05),
    },
    "moss.button.danger.outlined.border": {
      type: "solid",
      value: "null",
    },
    "moss.button.danger.outlined.text": {
      type: "solid",
      value: rgb(252, 165, 165),
    },

    "moss.button.danger.soft.background": {
      type: "solid",
      value: rgba(239, 68, 68, 0.1),
    },
    "moss.button.danger.soft.border": {
      type: "solid",
      value: "null",
    },
    "moss.button.danger.soft.text": {
      type: "solid",
      value: rgb(252, 165, 165),
    },

    "moss.button.danger.ghost.background": {
      type: "solid",
      value: rgba(239, 68, 68, 0.1),
    },
    "moss.button.danger.ghost.border": {
      type: "solid",
      value: "null",
    },
    "moss.button.danger.ghost.text": {
      type: "solid",
      value: rgb(252, 165, 165),
    },

    "moss.button.neutral.solid.background": {
      type: "gradient",
      value: linearGradient(null, { color: rgb(113, 113, 122) }, { color: rgb(82, 82, 91), percentage: 0 }),
    },
    "moss.button.neutral.solid.border": {
      type: "solid",
      value: rgb(113, 113, 122),
    },
    "moss.button.neutral.solid.text": {
      type: "solid",
      value: "white",
    },

    "moss.button.neutral.outlined.background": {
      type: "solid",
      value: rgba(113, 113, 122, 0.05),
    },
    "moss.button.neutral.outlined.border": {
      type: "solid",
      value: "null",
    },
    "moss.button.neutral.outlined.text": {
      type: "solid",
      value: rgb(212, 212, 216),
    },

    "moss.button.neutral.soft.background": {
      type: "solid",
      value: rgba(113, 113, 122, 0.1),
    },
    "moss.button.neutral.soft.border": {
      type: "solid",
      value: "null",
    },
    "moss.button.neutral.soft.text": {
      type: "solid",
      value: rgb(212, 212, 216),
    },

    "moss.button.neutral.ghost.background": {
      type: "solid",
      value: rgba(113, 113, 122, 0.1),
    },
    "moss.button.neutral.ghost.border": {
      type: "solid",
      value: "null",
    },
    "moss.button.neutral.ghost.text": {
      type: "solid",
      value: rgb(212, 212, 216),
    },

    "moss.button.background.danger": {
      type: "variable",
      value: "moss.red.5",
    },
    "moss.button.background.danger.hover": {
      type: "variable",
      value: "moss.red.3",
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
      value: rgb(39, 39, 42),
    },
    "moss.controls.plain.text": {
      type: "solid",
      value: "white",
    },
    "moss.controls.outlined.bg": {
      type: "solid",
      value: rgba(39, 39, 42, 0.5),
    },
    "moss.controls.outlined.border": {
      type: "solid",
      value: rgb(39, 39, 42),
    },
    "moss.controls.outlined.text": {
      type: "solid",
      value: "white",
    },
    "moss.controls.placeholder": {
      type: "solid",
      value: rgb(82, 82, 91),
    },
    "moss.select.bg.outlined": {
      type: "solid",
      value: rgb(24, 24, 27),
    },
    "moss.select.border.outlined": {
      type: "solid",
      value: rgb(39, 39, 42),
    },
    "moss.select.text.outlined": {
      type: "solid",
      value: "white",
    },
    "moss.select.hover.bg": {
      type: "solid",
      value: rgb(39, 39, 42),
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
    "moss.notification.header.color": {
      type: "variable",
      value: "moss.gray.3",
    },
    "moss.notification.bg": {
      type: "variable",
      value: "moss.gray.3",
    },
    "moss.notification.text": {
      type: "variable",
      value: "moss.gray.14",
    },
    "moss.notification.link.text": {
      type: "variable",
      value: "moss.blue.9",
    },
    "moss.notification.button.outline": {
      type: "variable",
      value: "moss.gray.7",
    },
    "moss.notification.button.hover": {
      type: "variable",
      value: "moss.gray.4",
    },
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
