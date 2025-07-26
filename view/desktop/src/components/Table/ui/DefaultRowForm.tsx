import { HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Table } from "@tanstack/react-table";

import { ParameterData } from "../types";

interface DefaultRowFormProps extends Omit<HTMLAttributes<HTMLFormElement>, "onInput"> {
  table: Table<ParameterData>;
  onInput: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export const DefaultAddNewRowForm = ({ className, table, onInput, ...props }: DefaultRowFormProps) => {
  return (
    <form
      id={`${table.options.meta?.tableId}-AddNewRowForm`}
      onSubmit={(e) => e.preventDefault()}
      className={cn("relative flex", className)}
      {...props}
    >
      {table.getVisibleFlatColumns().map((cell) => {
        const isLastColumn = cell.getIsLastColumn();
        const renderInput = cell.id !== "actions" && cell.id !== "checkbox";

        return (
          <div
            role="cell"
            key={cell.id}
            className={cn("border-r border-(--moss-border-color)", isLastColumn && "border-r-0")}
            style={{ width: cell.getSize() }}
          >
            {renderInput && (
              <input
                form={`${table.options.meta?.tableId}-AddNewRowForm`}
                placeholder={cell.id}
                className="w-full px-2 py-1.5 placeholder:text-(--moss-table-add-row-form-text) focus:outline-1 focus:outline-(--moss-primary)"
                onInput={onInput}
              />
            )}
          </div>
        );
      })}
    </form>
  );
};
