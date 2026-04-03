import type { SerializedDockview } from "moss-tabs";

import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

export const remapEnvironmentIdInDockviewLayout = ({
  oldEnvId,
  newEnvId,
  newTabIcon,
}: {
  oldEnvId: string;
  newEnvId: string;
  newTabIcon: string;
}) => {
  if (!oldEnvId || !newEnvId) {
    return;
  }

  const { api, setGridState } = useTabbedPaneStore.getState();
  if (!api) return;

  const layoutWithNewId = remapEnvironmentIdInSerializedDockview(api.toJSON() as SerializedDockview, {
    oldEnvId,
    newEnvId,
    newTabIcon,
  });
  api.fromJSON(layoutWithNewId);
  setGridState(layoutWithNewId);
};

function remapEnvironmentIdInSerializedDockview(
  state: SerializedDockview,
  { oldEnvId, newEnvId, newTabIcon }: { oldEnvId: string; newEnvId: string; newTabIcon: string }
): SerializedDockview {
  const idMap = new Map<string, string>([[oldEnvId, newEnvId]]);

  const clone = JSON.parse(JSON.stringify(state)) as SerializedDockview & {
    panels?: Record<string, unknown>;
  };

  const panels = clone.panels;
  if (panels && typeof panels === "object" && !Array.isArray(panels)) {
    const newPanels: Record<string, unknown> = {};

    for (const [panelKey, panelVal] of Object.entries(panels)) {
      const isTarget = panelKey === oldEnvId;
      const newKey = isTarget ? newEnvId : panelKey;
      const panel = panelVal && typeof panelVal === "object" ? { ...(panelVal as object) } : panelVal;

      if (isTarget && panel && typeof panel === "object") {
        const p = panel as Record<string, unknown>;
        p.id = newEnvId;

        const params = p.params;
        if (params && typeof params === "object") {
          const pr = { ...(params as Record<string, unknown>) };
          pr.tabIcon = newTabIcon;
          p.params = pr;
        }
      }

      newPanels[newKey] = panel;
    }

    clone.panels = newPanels as SerializedDockview["panels"];
  }

  remapStringIdsInValue(clone, idMap, new Set(["panels"]));

  return clone;
}

function remapStringIdsInValue(value: unknown, idMap: Map<string, string>, skipObjectKeys: Set<string>): void {
  if (value === null || value === undefined) {
    return;
  }
  if (typeof value === "string") {
    return;
  }
  if (Array.isArray(value)) {
    for (let i = 0; i < value.length; i++) {
      const el = value[i];
      if (typeof el === "string" && idMap.has(el)) {
        value[i] = idMap.get(el)!;
      } else {
        remapStringIdsInValue(el, idMap, skipObjectKeys);
      }
    }
    return;
  }
  if (typeof value === "object") {
    const obj = value as Record<string, unknown>;
    for (const key of Object.keys(obj)) {
      if (skipObjectKeys.has(key)) {
        continue;
      }
      const v = obj[key];
      if (typeof v === "string" && idMap.has(v)) {
        obj[key] = idMap.get(v)!;
      } else {
        remapStringIdsInValue(v, idMap, skipObjectKeys);
      }
    }
  }
}
