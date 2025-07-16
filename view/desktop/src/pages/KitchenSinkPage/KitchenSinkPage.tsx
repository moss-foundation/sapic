import React, { useState } from "react";
import { useTranslation } from "react-i18next";

import { ActionMenu } from "@/components";
import { ButtonNeutralOutlined } from "@/components/ButtonNeutralOutlined";
import { ButtonPrimary } from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { DataTable } from "@/components/Table/DataTable";
import { TestData } from "@/components/Table/types";
import { DefaultInputCell } from "@/components/Table/ui/DefaultCellInput";
import { InputTemplating } from "@/components/InputTemplating";
import {
  editorContextItems,
  generateItems,
  runConfigItems,
  runOptionsItems,
  runSelectorItems,
} from "@/data/actionMenuMockData";
import { invokeMossCommand } from "@/lib/backend/platfrom.ts";
import { Icon, Icons } from "@/lib/ui";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";
import { createColumnHelper, Table } from "@tanstack/react-table";

import * as iconsNames from "../../assets/icons";
import testData from "../../components/Table/testData.json";
import testData2 from "../../components/Table/testData2.json";
import { KitchenSinkSection } from "./KitchenSinkSection";

export const KitchenSink = () => {
  return <ComponentGallery />;
};

const ComponentGallery = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [data, setData] = React.useState<number | null>(null);

  React.useEffect(() => {
    const fetchData = async () => {
      setData(Math.floor(Math.random() * 100));
    };

    fetchData();
  }, []);

  return (
    <div className="mx-auto max-w-6xl space-y-10">
      {/* Random Data Display */}
      {data !== null && (
        <div className="rounded-lg bg-blue-50 p-4 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300">
          <p className="font-medium">
            {t("receivedData")}: {data}
          </p>
        </div>
      )}

      {/* Table Components */}
      <KitchenSinkSection header="Table" description="A table powered by Tanstack Table">
        <ExampleTable />
        <ExampleTable2 />
      </KitchenSinkSection>

      {/* Action Menu Components */}
      <KitchenSinkSection
        header="Action Menus"
        description="Various implementations of the ActionMenu component with different configurations."
      >
        <div>
          <h3 className="mb-4 text-xl font-medium text-gray-700 dark:text-gray-200">Standard Menu Triggers</h3>
          <div className="flex flex-wrap gap-4">
            {/* Generate Menu Button */}
            <ActionMenu.Root>
              <ActionMenu.Trigger asChild>
                <button className="w-fit cursor-pointer rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600">
                  Generate Menu
                </button>
              </ActionMenu.Trigger>
              <ActionMenu.Portal>
                <ActionMenu.Content>{generateItems.map((item) => renderActionMenuItem(item))}</ActionMenu.Content>
              </ActionMenu.Portal>
            </ActionMenu.Root>

            {/* Run Configurations Button */}
            <ActionMenu.Root>
              <ActionMenu.Trigger asChild>
                <button className="w-fit cursor-pointer rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600">
                  Run Configurations
                </button>
              </ActionMenu.Trigger>
              <ActionMenu.Portal>
                <ActionMenu.Content>{runConfigItems.map((item) => renderActionMenuItem(item))}</ActionMenu.Content>
              </ActionMenu.Portal>
            </ActionMenu.Root>

            {/* Run Options Button */}
            <ActionMenu.Root>
              <ActionMenu.Trigger asChild>
                <button className="w-fit cursor-pointer rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600">
                  Run Options
                </button>
              </ActionMenu.Trigger>
              <ActionMenu.Portal>
                <ActionMenu.Content>{runOptionsItems.map((item) => renderActionMenuItem(item))}</ActionMenu.Content>
              </ActionMenu.Portal>
            </ActionMenu.Root>

            {/* Run Selector Button */}
            <ActionMenu.Root>
              <ActionMenu.Trigger asChild>
                <button className="w-fit cursor-pointer rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600">
                  Run Selector
                </button>
              </ActionMenu.Trigger>
              <ActionMenu.Portal>
                <ActionMenu.Content>{runSelectorItems.map((item) => renderActionMenuItem(item))}</ActionMenu.Content>
              </ActionMenu.Portal>
            </ActionMenu.Root>
          </div>
        </div>
        <div>
          <h3 className="mb-4 text-xl font-medium text-gray-700 dark:text-gray-200">Context Actions Button</h3>
          <div>
            <ActionMenu.Root>
              <ActionMenu.Trigger asChild openOnRightClick>
                <button className="w-fit cursor-pointer rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600">
                  Show Context Actions
                </button>
              </ActionMenu.Trigger>
              <ActionMenu.Portal>
                <ActionMenu.Content>{editorContextItems.map((item) => renderActionMenuItem(item))}</ActionMenu.Content>
              </ActionMenu.Portal>
            </ActionMenu.Root>
          </div>
        </div>
      </KitchenSinkSection>

      {/* Button Components Section */}
      <KitchenSinkSection
        header="Button Components"
        description="Various button states and variants available in the application."
      >
        <div className="flex gap-2">
          <ButtonPrimary>ButtonPrimary</ButtonPrimary>
          <ButtonPrimary disabled>ButtonPrimaryDisabled</ButtonPrimary>
        </div>
        <div className="flex gap-2">
          <ButtonNeutralOutlined>ButtonNeutralOutlined</ButtonNeutralOutlined>
          <ButtonNeutralOutlined disabled>ButtonNeutralOutlinedDisabled</ButtonNeutralOutlined>
        </div>
      </KitchenSinkSection>

      {/* Input Templating Section */}
      <KitchenSinkSection
        header="Input Templating"
        description="Input component with template variable highlighting using double curly braces syntax."
      >
        <InputTemplatingDemo />
      </KitchenSinkSection>

      {/* Command Section */}
      <KitchenSinkSection
        header="Command Example"
        description="Demonstrates invoking a command through the platform API."
      >
        <button
          className="rounded-md bg-indigo-600 px-4 py-2 font-medium text-white shadow-sm transition-colors hover:bg-indigo-700 focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:outline-none dark:bg-indigo-500 dark:hover:bg-indigo-600"
          onClick={() => {
            invokeMossCommand("example.generateLog", {});
          }}
        >
          Example Command
        </button>
      </KitchenSinkSection>

      {/* Icons */}
      <KitchenSinkSection header="Icons" description="Various icons available in the application.">
        <div className="grid grid-cols-6 gap-y-2">
          {Object.keys(iconsNames).map((value) => (
            <div key={value} className="flex flex-col items-center gap-2">
              <Icon icon={value as Icons} />
              <span className="cursor-text rounded px-1 select-text hover:bg-gray-100 dark:hover:bg-gray-700">
                {value}
              </span>
            </div>
          ))}
        </div>
      </KitchenSinkSection>
    </div>
  );
};

const InputTemplatingDemo = () => {
  const [value, setValue] = useState("Hello {{name}}, your order {{orderId}} is ready!");
  const [variables, setVariables] = useState<string[]>([]);

  const handleTemplateChange = (newValue: string, extractedVariables: string[]) => {
    setValue(newValue);
    setVariables(extractedVariables);
  };

  return (
    <div className="space-y-4">
      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
          Template Input (try typing {"{{"} variable {"}}"} syntax):
        </label>
        <InputTemplating
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onTemplateChange={handleTemplateChange}
          placeholder="Type something with {{variables}} here..."
          className="w-full"
          size="md"
        />
      </div>

      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Size Variants:</label>
        <div className="space-y-2">
          <InputTemplating placeholder="Small (sm) - default" className="w-full" size="sm" />
          <InputTemplating placeholder="Medium (md)" className="w-full" size="md" />
        </div>
      </div>

      {variables.length > 0 && (
        <div className="mt-4 rounded-md bg-gray-50 p-3 dark:bg-gray-800">
          <h4 className="mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">Detected Variables:</h4>
          <div className="flex flex-wrap gap-2">
            {variables.map((variable, index) => (
              <span
                key={index}
                className="rounded bg-blue-100 px-2 py-1 text-sm text-blue-800 dark:bg-blue-900 dark:text-blue-200"
              >
                {variable}
              </span>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

const columnHelper = createColumnHelper<TestData>();
const columns = [
  columnHelper.display({
    id: "checkbox",
    header: ({ table }) => (
      <CheckboxWithLabel
        checked={table.getIsAllPageRowsSelected()}
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
      />
    ),
    cell: ({ row }) => (
      <CheckboxWithLabel
        disabled={!row.getCanSelect()}
        checked={row.getIsSelected()}
        onCheckedChange={row.getToggleSelectedHandler()}
      />
    ),
    enableSorting: false,
    enableResizing: false,
    size: 40,
  }),
  columnHelper.accessor("key", {
    header: () => "key",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 60,
  }),
  columnHelper.accessor("value", {
    header: () => "value",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("type", {
    header: () => "type",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("description", {
    header: () => "description",
    cell: (info) => <DefaultInputCell info={info} />,
    meta: {
      isGrow: true,
    },
    minSize: 100,
  }),
  columnHelper.accessor("global_value", {
    header: () => "Global value",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("local_value", {
    header: () => "Local value",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.display({
    id: "actions",
    header: () => "Actions",
    cell: () => (
      <div className="flex items-center justify-center gap-1">
        <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
          <Icon icon="AddToVcs" />
        </button>

        <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
          <Icon icon="RemoveCircle" />
        </button>
        <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
          <Icon icon="ConfigMap" />
        </button>
      </div>
    ),
    enableSorting: false,
    enableResizing: false,
    size: 90,
  }),
];

const ExampleTable = () => {
  const [tableApi, setTableApi] = useState<Table<TestData> | null>(null);

  const handleTableApiSet = (tableApi: Table<TestData>) => {
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
