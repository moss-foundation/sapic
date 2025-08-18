import { useState } from "react";

import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { columns, DataTable, ParameterData } from "@/components/Table";
import testData from "@/components/Table/testData.json";
import testData2 from "@/components/Table/testData2.json";
import { Table } from "@tanstack/react-table";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const TableSection = () => {
  return (
    <KitchenSinkSection header="Table" description="A table powered by Tanstack Table">
      <ExampleTable />
      <ExampleTable2 />
    </KitchenSinkSection>
  );
};

const ExampleTable = () => {
  const [tableApi, setTableApi] = useState<Table<ParameterData> | null>(null);

  const handleTableApiSet = (tableApi: Table<ParameterData>) => {
    setTableApi(tableApi);
  };

  return (
    <>
      <h2 className="mb-4 text-xl font-bold text-gray-800 dark:text-gray-100">columns visibility</h2>
      <div className="flex items-center gap-2">
        {tableApi?.getAllLeafColumns().map((column) => (
          <CheckboxWithLabel
            key={column.id}
            label={column.id}
            checked={column.getIsVisible()}
            onCheckedChange={() => column.toggleVisibility()}
          />
        ))}
      </div>
      <DataTable columns={columns} data={testData} onTableApiSet={handleTableApiSet} />
    </>
  );
};

const ExampleTable2 = () => {
  return <DataTable columns={columns} data={testData2} />;
};
