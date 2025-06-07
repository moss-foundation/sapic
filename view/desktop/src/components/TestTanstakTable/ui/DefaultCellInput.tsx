import { useState } from "react";

import { cn } from "@/utils";
import { CellContext } from "@tanstack/react-table";

import { TestData } from "../types";

export const TestTableInputCell = ({
  info,
}: {
  info: CellContext<TestData, number | string> & { focusOnMount?: boolean };
}) => {
  const [value, setValue] = useState(info.getValue());
  const isSelected = info.row.getIsSelected();

  const onBlur = () => {
    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  };

  return (
    <input
      className={cn(
        "w-full truncate px-2 py-1.5 focus:outline-1 focus:outline-(--moss-primary) disabled:text-(--moss-gray-1)/50",
        {
          "opacity-60": !isSelected,
        }
      )}
      value={value}
      onChange={(e) => setValue(e.target.value)}
      autoFocus={info.focusOnMount}
      onBlur={onBlur}
      placeholder={info.column.id}
    />
  );
};
