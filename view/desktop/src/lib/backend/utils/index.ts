import { GroupPanelViewState, Orientation, SerializedDockview, SerializedGridObject } from "@/lib/moss-tabs/src";
import { EditorPartState } from "@repo/moss-workspace";

export function mapEditorPartStateToSerializedDockview(editorState: EditorPartState): SerializedDockview {
  return {
    grid: {
      root: editorState.grid.root,
      height: editorState.grid.height,
      width: editorState.grid.width,
      orientation: editorState.grid.orientation === "HORIZONTAL" ? Orientation.HORIZONTAL : Orientation.VERTICAL,
    },
    panels: editorState.panels,
    activeGroup: editorState.activeGroup,
  };
}

const serializedDockviewGridRootDataToEditor = (
  data: GroupPanelViewState | SerializedGridObject<GroupPanelViewState>[]
) => {
  if (Array.isArray(data)) {
    return data.map((item) => ({
      ...item,
      data: serializedDockviewGridRootDataToEditor(item.data),
    }));
  }

  return data;
};

export function mapSerializedDockviewGridRootDataToEditorPartStateGridRootData(
  serializedDockview: SerializedDockview
): EditorPartState {
  return {
    grid: {
      root: {
        data: serializedDockviewGridRootDataToEditor(serializedDockview.grid.root.data),
        size: serializedDockview.grid.root.size ?? 0,
        type: serializedDockview.grid.root.type,
      },
      height: serializedDockview.grid.height,
      width: serializedDockview.grid.width,
      orientation: serializedDockview.grid.orientation,
    },
    panels: serializedDockview.panels,
    activeGroup: serializedDockview.activeGroup,
  };
}
