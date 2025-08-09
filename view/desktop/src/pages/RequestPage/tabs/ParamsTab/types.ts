import { CellContext } from "@tanstack/react-table";

export interface ExtendedCellContext<TData, TValue> extends CellContext<TData, TValue> {
  focusOnMount?: boolean;
}
