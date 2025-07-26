import { ColumnDef, RowData, Table } from "@tanstack/react-table";

export interface DataTableProps<TData> {
  columns: {
    [K in keyof TData]: ColumnDef<TData, TData[K]>;
  }[keyof TData][];
  data: TData[];
  onTableApiSet?: (table: Table<TData>) => void;
}

declare module "@tanstack/react-table" {
  interface TableMeta<TData extends RowData> {
    tableId: string;
    tableType: "ActionsTable";
    updateData: (rowIndex: number, columnId: string, value: unknown) => void;
  }
  interface ColumnMeta<TData extends RowData, TValue> {
    isGrow?: boolean;
    widthPercentage?: number;
  }
}

export interface ParameterData {
  order: number;
  id: string;
  key: string;
  value: string;
  type: string;
  description: string;
  global_value: string;
  local_value: number;
  properties: {
    disabled: boolean;
  };
}

export interface TableRowDnDData {
  type: "TableRow";
  data: {
    tableType: string;
    tableId: string;
    row: ParameterData;
    isSelected: boolean;
  };
}
