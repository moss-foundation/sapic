import type { SerializedDockview } from "moss-tabs";

import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { BatchCreateResourceOutput } from "@repo/moss-project";

import { ResourceNode } from "../../types";

export type CrossProjectResourceRemapEntry = {
  newId: string;
  destProjectId: string;
};

export const remapOldIdsForDockviewLayout = ({
  allFlatSourceResourceNodes,
  batchCreateResourceOutput,
  destProjectId,
}: {
  allFlatSourceResourceNodes: ResourceNode[];
  batchCreateResourceOutput: BatchCreateResourceOutput;
  destProjectId: string;
}) => {
  if (
    allFlatSourceResourceNodes.length === 0 ||
    batchCreateResourceOutput.resources.length === 0 ||
    allFlatSourceResourceNodes.length !== batchCreateResourceOutput.resources.length
  ) {
    return;
  }

  const remap = new Map<string, CrossProjectResourceRemapEntry>();
  allFlatSourceResourceNodes.forEach((node, index) => {
    remap.set(node.id, {
      newId: batchCreateResourceOutput.resources[index].id,
      destProjectId,
    });
  });

  const { api, setGridState } = useTabbedPaneStore.getState();
  if (!api) return;

  const layoutWithNewIds = remapResourceIdsInSerializedDockview(api.toJSON() as SerializedDockview, remap);
  api.fromJSON(layoutWithNewIds);
  setGridState(layoutWithNewIds);
};

/**
 * Remaps dockview panel ids and EndpointView / FolderSettingsView params after resources
 * were deleted and recreated with new ids. Also replaces id strings elsewhere in the layout
 * (e.g. grid leaf references).
 */
export function remapResourceIdsInSerializedDockview(
  state: SerializedDockview,
  remap: Map<string, CrossProjectResourceRemapEntry>
): SerializedDockview {
  if (remap.size === 0) {
    return state;
  }

  const idOnly = new Map<string, string>();
  remap.forEach((entry, oldId) => {
    idOnly.set(oldId, entry.newId);
  });

  const clone = JSON.parse(JSON.stringify(state)) as SerializedDockview & {
    panels?: Record<string, unknown>;
  };

  const panels = clone.panels;
  if (panels && typeof panels === "object" && !Array.isArray(panels)) {
    const newPanels: Record<string, unknown> = {};

    for (const [panelKey, panelVal] of Object.entries(panels)) {
      const entry = remap.get(panelKey);
      const newKey = entry?.newId ?? panelKey;
      const panel = panelVal && typeof panelVal === "object" ? { ...(panelVal as object) } : panelVal;

      if (panel && typeof panel === "object") {
        const p = panel as Record<string, unknown>;
        if (entry) {
          p.id = entry.newId;
        } else if (typeof p.id === "string" && idOnly.has(p.id)) {
          p.id = idOnly.get(p.id);
        }

        const params = p.params;
        if (params && typeof params === "object") {
          const pr = { ...(params as Record<string, unknown>) };
          if (typeof pr.resourceId === "string" && idOnly.has(pr.resourceId)) {
            pr.resourceId = idOnly.get(pr.resourceId)!;
          }
          if (entry) {
            pr.projectId = entry.destProjectId;
          }
          const node = pr.node;
          if (entry && node && typeof node === "object") {
            const n = { ...(node as Record<string, unknown>) };
            n.id = entry.newId;
            pr.node = n;
          }
          p.params = pr;
        }
      }

      newPanels[newKey] = panel;
    }

    clone.panels = newPanels as SerializedDockview["panels"];
  }

  remapStringIdsInValue(clone, idOnly, new Set(["panels"]));

  return clone;
}

function remapStringIdsInValue(value: unknown, idOnly: Map<string, string>, skipObjectKeys: Set<string>): void {
  if (value === null || value === undefined) {
    return;
  }
  if (typeof value === "string") {
    return;
  }
  if (Array.isArray(value)) {
    for (let i = 0; i < value.length; i++) {
      const el = value[i];
      if (typeof el === "string" && idOnly.has(el)) {
        value[i] = idOnly.get(el)!;
      } else {
        remapStringIdsInValue(el, idOnly, skipObjectKeys);
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
      if (typeof v === "string" && idOnly.has(v)) {
        obj[key] = idOnly.get(v)!;
      } else {
        remapStringIdsInValue(v, idOnly, skipObjectKeys);
      }
    }
  }
}
