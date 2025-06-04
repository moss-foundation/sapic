import { Table } from "@tanstack/react-table";
import { useLayoutEffect } from "react";
import { calculateTableSizing } from "../calculateTableSizing";
import { TestData } from "../DataTable";

interface AdjustColumnsWithoutSizesProps {
  table: Table<TestData>;
  tableContainerRef: React.RefObject<HTMLDivElement>;
}

export const useAdjustColumnsWithoutSizes = ({ table, tableContainerRef }: AdjustColumnsWithoutSizesProps) => {
  const headers = table.getFlatHeaders();

  //this should only be called on mount
  useLayoutEffect(() => {
    if (tableContainerRef.current) {
      const initialColumnSizing = calculateTableSizing(headers, tableContainerRef.current?.clientWidth);
      table.setColumnSizing(initialColumnSizing);
    }
  }, []);
};
