import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { TableRowDnDData } from "./types";

export const isTableRow = (source: ElementDragPayload) => {
  return source.data.type === "TableRow";
};

export function getTableRowData(source: ElementDragPayload): TableRowDnDData["data"];
export function getTableRowData(location: DragLocationHistory): TableRowDnDData["data"];
export function getTableRowData(input: ElementDragPayload | DragLocationHistory): TableRowDnDData["data"] {
  if ("data" in input) {
    return input.data.data as TableRowDnDData["data"];
  }
  return input.current.dropTargets[0].data.data as TableRowDnDData["data"];
}
