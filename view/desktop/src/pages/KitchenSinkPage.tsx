import React, { useState } from "react";
import { useTranslation } from "react-i18next";

import { ActionMenu } from "@/components";
import { ButtonNeutralOutlined } from "@/components/ButtonNeutralOutlined";
import { ButtonPrimary } from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import SelectOutlined from "@/components/SelectOutlined";
import { DataTable } from "@/components/Table/DataTable";
import { TestData } from "@/components/Table/types";
import { DefaultInputCell } from "@/components/Table/ui/DefaultCellInput";
import {
  editorContextItems,
  generateItems,
  runConfigItems,
  runOptionsItems,
  runSelectorItems,
  themeItems,
} from "@/data/actionMenuMockData";
import { invokeMossCommand } from "@/lib/backend/platfrom.ts";
import { Icon, Icons, Scrollbar } from "@/lib/ui";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";
import { createColumnHelper, Table } from "@tanstack/react-table";

import * as iconsNames from "../assets/icons";
import testData from "../components/Table/testData.json";
import testData2 from "../components/Table/testData2.json";

export const KitchenSink = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  return (
    <div className="flex h-full flex-col bg-gray-50 dark:bg-stone-900">
      <header className="border-b border-gray-200 bg-white p-6 shadow-sm dark:border-stone-800 dark:bg-stone-950">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">{t("home")}</h1>
        <p className="mt-2 text-gray-500 dark:text-gray-400">Component Gallery & Storybook</p>
      </header>

      <Scrollbar className="flex-1 overflow-auto p-6">
        <ComponentGallery />
      </Scrollbar>
    </div>
  );
};

const ComponentGallery = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [data, setData] = React.useState<number | null>(null);

  // Action Menu States
  const [selectedTheme, setSelectedTheme] = React.useState("light");

  React.useEffect(() => {
    const fetchData = async () => {
      setData(Math.floor(Math.random() * 100));
    };

    fetchData();
  }, []);

  const handleItemSelect = (value: string) => {
    console.log(`Selected: ${value}`);

    // Handle theme selection
    setSelectedTheme(value);
  };

  return (
    <div className="mx-auto max-w-6xl space-y-10">
      <ExampleTable />
      <ExampleTable2 />
      {/* Random Data Display */}
      {data !== null && (
        <div className="rounded-lg bg-blue-50 p-4 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300">
          <p className="font-medium">
            {t("receivedData")}: {data}
          </p>
        </div>
      )}
      {/* Action Menu Components */}
      <section className="rounded-xl bg-white p-6 shadow-md dark:bg-stone-800">
        <h2 className="mb-4 text-2xl font-bold text-gray-800 dark:text-gray-100">Action Menus</h2>
        <p className="mb-6 text-gray-600 dark:text-gray-300">
          Various implementations of the ActionMenu component with different configurations.
        </p>

        <div className="mb-10 space-y-8">
          <div>
            <h3 className="mb-4 text-xl font-medium text-gray-700 dark:text-gray-200">Standard Menu Triggers</h3>
            <div className="flex flex-wrap gap-4">
              {/* Context Actions Button */}
              <ActionMenu.Root>
                <ActionMenu.Trigger asChild>
                  <button className="w-fit cursor-pointer rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600">
                    Show Context Actions
                  </button>
                </ActionMenu.Trigger>
                <ActionMenu.Portal>
                  <ActionMenu.Content>
                    {editorContextItems.map((item) => renderActionMenuItem(item))}
                  </ActionMenu.Content>
                </ActionMenu.Portal>
              </ActionMenu.Root>

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

          {/* Theme Selector Dropdown */}
          <div>
            <h3 className="mb-4 text-xl font-medium text-gray-700 dark:text-gray-200">Dropdown Menu</h3>
            <div className="flex items-center gap-3 rounded-md bg-gray-100 p-4 dark:bg-gray-800/50">
              <span className="font-medium text-gray-700 dark:text-gray-300">Theme:</span>
              <div className="w-56">
                <SelectOutlined.Root value={selectedTheme} onValueChange={handleItemSelect}>
                  <SelectOutlined.Trigger />

                  <SelectOutlined.Content>
                    {themeItems.map((item) => {
                      if (item.type === "separator") {
                        return <SelectOutlined.Separator key={item.id} />;
                      }

                      return (
                        <SelectOutlined.Item key={item.id} value={item.value!}>
                          {item.label}
                        </SelectOutlined.Item>
                      );
                    })}
                  </SelectOutlined.Content>
                </SelectOutlined.Root>
              </div>
            </div>
          </div>
        </div>
      </section>
      Button Components
      <section className="rounded-xl bg-white p-6 shadow-md dark:bg-stone-800">
        <h2 className="mb-4 text-2xl font-bold text-gray-800 dark:text-gray-100">Button Components</h2>
        <p className="mb-6 text-gray-600 dark:text-gray-300">
          Various button states and variants available in the application.
        </p>
        <div className="flex gap-2">
          <ButtonPrimary>ButtonPrimary</ButtonPrimary>
          <ButtonPrimary disabled>ButtonPrimaryDisabled</ButtonPrimary>
        </div>
        <div className="flex gap-2">
          <ButtonNeutralOutlined>ButtonNeutralOutlined</ButtonNeutralOutlined>
          <ButtonNeutralOutlined disabled>ButtonNeutralOutlinedDisabled</ButtonNeutralOutlined>
        </div>
      </section>
      {/* Command Section */}
      <section className="rounded-xl bg-white p-6 shadow-md dark:bg-stone-800">
        <h2 className="mb-4 text-2xl font-bold text-gray-800 dark:text-gray-100">Command Example</h2>
        <p className="mb-4 text-gray-600 dark:text-gray-300">
          Demonstrates invoking a command through the platform API.
        </p>
        <button
          className="rounded-md bg-indigo-600 px-4 py-2 font-medium text-white shadow-sm transition-colors hover:bg-indigo-700 focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:outline-none dark:bg-indigo-500 dark:hover:bg-indigo-600"
          onClick={() => {
            invokeMossCommand("example.generateLog", {});
          }}
        >
          Example Command
        </button>
      </section>
      {/* Icons */}
      <section className="rounded-xl bg-white p-6 shadow-md dark:bg-stone-800">
        <h2 className="mb-4 text-2xl font-bold text-gray-800 dark:text-gray-100">Icons</h2>
        <p className="mb-6 text-gray-600 dark:text-gray-300">Various icons available in the application.</p>
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
      </section>
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
        {tableApi
          ?.getAllLeafColumns()
          .map((column) => (
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
