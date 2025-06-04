import { Header, Table } from "@tanstack/react-table";
import { RefObject, useLayoutEffect } from "react";
import { TestData } from "../DataTable";

interface AdjustColumnsWithoutSizesProps {
  table: Table<TestData>;
  tableContainerRef: RefObject<HTMLDivElement>;
}

export const useAdjustColumnsWithoutSizes = ({ table, tableContainerRef }: AdjustColumnsWithoutSizesProps) => {
  const headers = table.getFlatHeaders();

  //This should only be called on mount. This takes all the columns without sizes and distributes the width proportionally.
  useLayoutEffect(() => {
    if (tableContainerRef.current) {
      const initialColumnSizing = calculateTableSizing(headers, tableContainerRef.current?.clientWidth);
      table.setColumnSizing(initialColumnSizing);
    }
  }, []);
};

//got calculations of table sizing from here: https://github.com/TanStack/table/discussions/3192#discussioncomment-11896090

function getSize(size = 100, max = Number.MAX_SAFE_INTEGER, min = 40) {
  return Math.max(Math.min(size, max), min);
}

/**
 * Calculates the sizing of table columns and distributes available width proportionally.
 * This function acts as an extension for TanStack Table, ensuring proper column sizing
 * based on provided metadata, including `isGrow`, `widthPercentage`, and size constraints.
 *
 * @template DataType - The generic type of data used in the table rows.
 *
 * @param {Header<DataType, unknown>[]} columns - An array of column headers. Each header contains
 *   metadata about the column, including size, constraints, and growth behavior.
 * @param {number} totalWidth - The total width available for the table, including padding and margins.
 *
 * @returns {Record<string, number>} An object mapping column IDs to their calculated sizes.
 */
const calculateTableSizing = <DataType>(
  columns: Header<DataType, unknown>[],
  totalWidth: number
): Record<string, number> => {
  let totalAvailableWidth = totalWidth;
  let totalIsGrow = 0;

  columns.forEach((header) => {
    const column = header.column.columnDef;
    if (!column.size) {
      if (!column.meta?.isGrow) {
        let calculatedSize = 100;
        if (column?.meta?.widthPercentage) {
          calculatedSize = column.meta.widthPercentage * totalWidth * 0.01;
        } else {
          calculatedSize = totalWidth / columns.length;
        }

        const size = getSize(calculatedSize, column.maxSize, column.minSize);

        column.size = size;
      }
    }

    if (column.meta?.isGrow) totalIsGrow += 1;
    else totalAvailableWidth -= getSize(column.size, column.maxSize, column.minSize);
  });

  const sizing: Record<string, number> = {};

  columns.forEach((header) => {
    const column = header.column.columnDef;
    if (column.meta?.isGrow) {
      let calculatedSize = 100;
      calculatedSize = Math.floor(totalAvailableWidth / totalIsGrow);
      const size = getSize(calculatedSize, column.maxSize, column.minSize);
      column.size = size;
    }

    sizing[`${column.id}`] = Number(column.size);
  });

  return sizing;
};
