import { ColumnDef, RowData, Table } from "@tanstack/react-table";

export interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[];
  data: TData[];
  onTableApiSet: (table: Table<TData>) => void;

  meta: {
    id: string;
    setData: (data: TData[]) => void;
  };
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

export interface TestData {
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
    row: TestData;
    isSelected: boolean;
  };
}
