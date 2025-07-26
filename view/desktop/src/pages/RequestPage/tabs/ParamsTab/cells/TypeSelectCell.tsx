import { useState } from "react";
import { ParameterData } from "@/components/Table";
import { ActionMenu } from "@/components";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { ExtendedCellContext } from "../types";

const DATA_TYPES = ["string", "number", "bool"];

const getTypeColor = (type: string) => {
  switch (type) {
    case "string":
      return "text-(--moss-requestpage-string-color)";
    case "number":
      return "text-(--moss-requestpage-number-color)";
    case "bool":
      return "text-(--moss-requestpage-bool-color)";
    default:
      return "text-(--moss-primary-text)";
  }
};

export const TypeSelectCell = ({ info }: { info: ExtendedCellContext<ParameterData, string> }) => {
  const [value, setValue] = useState(info.getValue());

  const handleTypeChange = (newType: string) => {
    setValue(newType);
    info.table.options.meta?.updateData(info.row.index, info.column.id, newType);
  };

  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <button
          className={cn(
            "flex w-full items-center justify-between px-2 py-1.5 text-sm transition-colors",
            "border-none bg-transparent",
            "focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-(--moss-primary)",
            "data-[state=open]:outline-2 data-[state=open]:-outline-offset-1 data-[state=open]:outline-(--moss-primary)",
            getTypeColor(value)
          )}
        >
          <span>{value}</span>
          <Icon icon="ChevronDown" className="h-3 w-3 cursor-pointer text-(--moss-requestpage-icon-color)" />
        </button>
      </ActionMenu.Trigger>
      <ActionMenu.Content>
        {DATA_TYPES.map((dataType) => (
          <ActionMenu.Item
            key={dataType}
            onClick={() => handleTypeChange(dataType)}
            className={cn(
              getTypeColor(dataType),
              value === dataType && "background-(--moss-secondary-background-hover) font-medium"
            )}
          >
            {dataType}
          </ActionMenu.Item>
        ))}
      </ActionMenu.Content>
    </ActionMenu.Root>
  );
};
