import { Row } from "@tanstack/react-table";
import { CheckedState } from "@radix-ui/react-checkbox";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { ParameterData } from "@/components/Table";

export const EnabledCheckboxCell = ({ row, table }: { row: Row<ParameterData>; table: any }) => {
  const enabled = !row.original.properties.disabled;

  const handleCheckedChange = (checked: CheckedState) => {
    const isChecked = checked === true;

    const updatedProperties = {
      ...row.original.properties,
      disabled: !isChecked,
    };

    // Use the table's meta updateData function to trigger re-renders
    table.options.meta?.updateData(row.index, "properties", updatedProperties);
  };

  return (
    <div className="flex items-center">
      <CheckboxWithLabel checked={enabled} onCheckedChange={handleCheckedChange} />
    </div>
  );
};
