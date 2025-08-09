import { Table } from "@tanstack/react-table";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { ParameterData } from "@/components/Table";

export const EnabledHeaderCheckbox = ({ table }: { table: Table<ParameterData> }) => {
  const allRows = table.getRowModel().rows;
  const enabledRows = allRows.filter((row) => !row.original.properties.disabled);
  const isAllEnabled = allRows.length > 0 && enabledRows.length === allRows.length;
  const isIndeterminate = enabledRows.length > 0 && enabledRows.length < allRows.length;

  const handleToggleAll = (checked: boolean) => {
    allRows.forEach((row, index) => {
      const updatedProperties = {
        ...row.original.properties,
        disabled: !checked,
      };

      // Update each row's data
      table.options.meta?.updateData(index, "properties", updatedProperties);
    });
  };

  return (
    <CheckboxWithLabel
      checked={isAllEnabled}
      indeterminate={isIndeterminate}
      onCheckedChange={(checked) => handleToggleAll(!!checked)}
    />
  );
};
