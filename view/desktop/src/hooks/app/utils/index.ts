import { GroupPanelViewState, Orientation, SerializedDockview, SerializedGridObject } from "moss-tabs";

import { EditorGridNode, EditorPartStateInfo } from "@repo/moss-workspace";

const mapSerializedRootToEditorRoot = (node: SerializedGridObject<GroupPanelViewState>): EditorGridNode => {
  if (node.type === "branch") {
    return {
      type: "branch",
      size: node.size ?? 0,
      data: (node.data as SerializedGridObject<GroupPanelViewState>[]).map(mapSerializedRootToEditorRoot),
    };
  }

  const leafData = node.data as GroupPanelViewState;
  return {
    type: "leaf",
    size: node.size ?? 0,
    data: {
      id: leafData.id,
      views: leafData.views,
      activeView: leafData.activeView ?? leafData.views[0],
    },
  };
};

const mapEditorOrientationToSerialized = (orientation: "HORIZONTAL" | "VERTICAL"): Orientation => {
  return orientation === "HORIZONTAL" ? Orientation.HORIZONTAL : Orientation.VERTICAL;
};

const mapSerializedOrientationToEditor = (orientation: Orientation): "HORIZONTAL" | "VERTICAL" => {
  return orientation === Orientation.HORIZONTAL ? "HORIZONTAL" : "VERTICAL";
};

export const mapEditorPartStateToSerializedDockview = (editor: EditorPartStateInfo): SerializedDockview => {
  const {
    panels,
    activeGroup,
    grid: { root, height, width, orientation },
  } = editor;

  return {
    panels,
    activeGroup,
    grid: {
      root,
      height,
      width,
      orientation: mapEditorOrientationToSerialized(orientation),
    },
  };
};

export const mapSerializedDockviewToEditorPartState = (dockview: SerializedDockview): EditorPartStateInfo => {
  const {
    panels,
    activeGroup,
    grid: { root, height, width, orientation },
  } = dockview;

  // Fixed type error: ensure panels values are cast to EditorPanelState and params are always defined
  return {
    panels: Object.fromEntries(
      Object.entries(panels).map(([key, panel]) => [
        key,
        {
          ...panel,
          params: panel.params ?? {},
        },
      ])
    ),
    activeGroup,
    grid: {
      height,
      width,
      orientation: mapSerializedOrientationToEditor(orientation),
      root: mapSerializedRootToEditorRoot(root),
    },
  };
};
