import { useState } from "react";
import { ParameterData } from "@/components/Table";
import { ExtendedCellContext } from "../types";

export const ParamInputCell = ({ info }: { info: ExtendedCellContext<ParameterData, string> }) => {
  const [value, setValue] = useState(info.getValue());
  const isDisabled = info.row.original.properties.disabled;

  const onBlur = () => {
    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  };

  return (
    <input
      className={`w-full truncate border-none bg-transparent px-2 py-1.5 placeholder-(--moss-requestpage-placeholder-color) focus:outline-1 focus:outline-(--moss-primary) ${
        isDisabled ? "text-(--moss-requestpage-text-disabled)" : "text-(--moss-primary-text)"
      }`}
      value={value}
      onChange={(e) => setValue(e.target.value)}
      autoFocus={info.focusOnMount}
      onBlur={onBlur}
      placeholder={info.column.id}
    />
  );
};
