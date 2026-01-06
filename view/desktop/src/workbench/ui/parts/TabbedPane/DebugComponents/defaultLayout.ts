import { DockviewApi } from "moss-tabs";

export const nextId = (() => {
  let counter = 0;

  return () => counter++;
})();

export function defaultConfig(api: DockviewApi) {
  const panel1 = api.addPanel({
    id: "KitchenSink",
    component: "KitchenSinkView",
    renderer: "onlyWhenVisible",
    title: "KitchenSink",
  });

  api.addPanel({
    id: "Settings",
    component: "SettingsView",
    title: "Settings",
    position: { referencePanel: panel1 },
  });

  api.addPanel({
    id: "Logs",
    component: "LogsView",
    title: "Logs",
    position: { referencePanel: panel1 },
  });

  const panel4 = api.addPanel({
    id: "panel_4",
    component: "DefaultView",
    title: "Panel 4",
    position: { referencePanel: panel1, direction: "right" },
  });

  const panel5 = api.addPanel({
    id: "panel_5",
    component: "DefaultView",
    title: "Panel 5",
    position: { referencePanel: panel4 },
  });

  const panel6 = api.addPanel({
    id: "panel_6",
    component: "DefaultView",
    title: "Panel 6",
    position: { referencePanel: panel5, direction: "below" },
  });

  const panel7 = api.addPanel({
    id: "panel_7",
    component: "DefaultView",
    title: "Panel 7",
    position: { referencePanel: panel6, direction: "left" },
  });

  api.addPanel({
    id: "panel8",
    component: "DefaultView",
    title: "Panel 8",
    position: { referencePanel: panel7, direction: "below" },
  });

  panel1.api.setActive();
}
