import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { TableRowDnDData } from "./types";

export const isTableRow = (source: ElementDragPayload) => {
  return source.data.type === "TableRow";
};

export const getTableRowData = (source: ElementDragPayload) => {
  return source.data.data as TableRowDnDData["data"];
};
