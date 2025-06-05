import React, { useState } from "react";
import { useTranslation } from "react-i18next";

import { ActionMenu } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import SelectOutlined from "@/components/SelectOutlined";
import { DataTable } from "@/components/TestTanstakTable/DataTable";
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
import { CellContext, createColumnHelper, Table } from "@tanstack/react-table";

import * as iconsNames from "../assets/icons";
import testData from "../components/TestTanstakTable/testData.json";
import testData2 from "../components/TestTanstakTable/testData2.json";

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

interface TestData {
  key: string;
  value: string;
  type: string;
  description: string;
  global_value: string;
  local_value: number;
  properties: {
    disabled: boolean;
  };
}

const columnHelper = createColumnHelper<TestData>();
const columns = [
  columnHelper.display({
    id: "checkbox",
    header: ({ table }) => (
      <div className="flex items-center justify-center">
        <CheckboxWithLabel
          checked={table.getIsAllPageRowsSelected()}
          onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
        />
      </div>
    ),
    cell: ({ row }) => (
      <div className="flex items-center justify-center">
        <CheckboxWithLabel
          disabled={!row.getCanSelect()}
          checked={row.getIsSelected()}
          onCheckedChange={row.getToggleSelectedHandler()}
        />
      </div>
    ),
    enableSorting: false,
    enableResizing: false,
    size: 40,
  }),
  columnHelper.accessor("key", {
    header: () => "key",
    cell: (info) => <TestTableInputCell info={info} />,
    minSize: 60,
  }),
  columnHelper.accessor("value", {
    header: () => "value",
    cell: (info) => <TestTableInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("type", {
    header: () => "type",
    cell: (info) => <TestTableInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("description", {
    header: () => "description",
    cell: (info) => <TestTableInputCell info={info} />,
    meta: {
      isGrow: true,
    },
    minSize: 100,
  }),
  columnHelper.accessor("global_value", {
    header: () => "global_value",
    cell: (info) => <TestTableInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("local_value", {
    header: () => "local_value",
    cell: (info) => <TestTableInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.display({
    id: "actions",
    header: ({}) => <div>Actions</div>,
    cell: ({}) => (
      <div className="flex items-center justify-center gap-1">
        <TableActionButton icon="Add" />
        <TableActionButton icon="Edit" />
        <TableActionButton icon="Delete" />
      </div>
    ),
    enableSorting: false,
    enableResizing: false,
    size: 90,
  }),
];

const TestTableInputCell = ({ info }: { info: CellContext<TestData, number | string> }) => {
  const [str, setStr] = useState(info.getValue());

  return (
    <input
      className="w-full truncate px-2 py-1.5 focus:outline-1 focus:outline-(--moss-primary) disabled:text-(--moss-gray-1)/50"
      value={str}
      onChange={(e) => setStr(e.target.value)}
    />
  );
};

const TableActionButton = ({ icon }: { icon: "Add" | "Edit" | "Delete" }) => {
  if (icon === "Add") {
    return (
      <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            fillRule="evenodd"
            clipRule="evenodd"
            d="M5 5.94999C6.14112 5.71836 7 4.70948 7 3.5C7 2.11929 5.88071 1 4.5 1C3.11929 1 2 2.11929 2 3.5C2 4.70948 2.85888 5.71836 4 5.94999L4 14.5C4 14.7761 4.22386 15 4.5 15C4.77614 15 5 14.7761 5 14.5L5 12H8.08535C8.29127 11.4174 8.84689 11 9.5 11H10.5002C10.814 10.5822 11 10.0628 11 9.5V8.94999C12.1411 8.71836 13 7.70948 13 6.5C13 5.11929 11.8807 4 10.5 4C9.11929 4 8 5.11929 8 6.5C8 7.70948 8.85888 8.71836 10 8.94999V9.5C10 10.3284 9.32843 11 8.5 11H5L5 5.94999ZM4.5 5C5.32843 5 6 4.32843 6 3.5C6 2.67157 5.32843 2 4.5 2C3.67157 2 3 2.67157 3 3.5C3 4.32843 3.67157 5 4.5 5ZM10.5 8C11.3284 8 12 7.32843 12 6.5C12 5.67157 11.3284 5 10.5 5C9.67157 5 9 5.67157 9 6.5C9 7.32843 9.67157 8 10.5 8Z"
            fill="#6C707E"
          />
          <path
            fillRule="evenodd"
            clipRule="evenodd"
            d="M12.5 9C12.7761 9 13 9.22386 13 9.5V12H15.5C15.7761 12 16 12.2239 16 12.5C16 12.7761 15.7761 13 15.5 13H13V15.5C13 15.7761 12.7761 16 12.5 16C12.2239 16 12 15.7761 12 15.5V13H9.5C9.22386 13 9 12.7761 9 12.5C9 12.2239 9.22386 12 9.5 12H12V9.5C12 9.22386 12.2239 9 12.5 9Z"
            fill="#3574F0"
          />
        </svg>
      </button>
    );
  }
  if (icon === "Edit") {
    return (
      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M4.64645 11.8535C4.45118 11.6583 4.45118 11.3417 4.64645 11.1464C4.84171 10.9511 5.15829 10.9511 5.35355 11.1464L7.5 13.2929L7.5 7.2929C7.5 7.01675 7.72386 6.79289 8 6.79289C8.27614 6.79289 8.5 7.01675 8.5 7.2929L8.5 13.2929L10.6464 11.1464C10.8417 10.9511 11.1583 10.9511 11.3536 11.1464C11.5488 11.3417 11.5488 11.6583 11.3536 11.8535L8.35355 14.8536C8.25978 14.9473 8.13261 15 8 15C7.86739 15 7.74021 14.9473 7.64645 14.8536L4.64645 11.8535Z"
                fill="#717171"
              />
              <path
                d="M6.5 7.29275L3.25218 5.43272C2.91678 5.24063 2.91678 4.75717 3.25218 4.56508L7.50138 2.13156C7.80948 1.95511 8.18808 1.95506 8.49622 2.13145L12.7477 4.56503C13.0833 4.75709 13.0833 5.24071 12.7477 5.43277L9.5 7.29178V8.4442L13.245 6.30051C14.2517 5.72433 14.2517 4.27347 13.245 3.69728L8.99355 1.2637C8.37726 0.910939 7.62006 0.911023 7.00386 1.26393L2.75466 3.69745C1.74845 4.27372 1.74845 5.72408 2.75466 6.30035L6.5 8.44531V7.29275Z"
                fill="#369650"
              />
            </svg>
          </button>
        </ActionMenu.Trigger>
        <ActionMenu.Portal>
          <ActionMenu.Content className="w-10">
            <ActionMenu.Item>Edit</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    );
  }
  if (icon === "Delete") {
    return (
      <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="16" height="16" fill="white" fillOpacity="0.01" />
          <path
            d="M14.5 13H5.99998C5.86742 13 5.74023 12.9473 5.64648 12.8536L1.14648 8.3536C0.951175 8.1583 0.951175 7.84175 1.14648 7.64645L5.64648 3.1464C5.74023 3.05265 5.86742 3 5.99998 3H14.5C14.7761 3 15 3.2239 15 3.5V12.5C15 12.7761 14.7761 13 14.5 13ZM6.20713 12H14V4H6.20713L2.20712 8L6.20713 12Z"
            fill="#717171"
          />
          <path
            d="M10.2071 8L12.5 5.70705L11.7929 5L9.5 7.29295L7.20715 5L6.5 5.70705L8.79295 8L6.5 10.2929L7.20715 11L8.35357 9.85353L9.5 8.70705L11.7929 11L12.5 10.2929L10.2071 8Z"
            fill="#DB3B4B"
          />
        </svg>
      </button>
    );
  }

  return null;
};

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
          <div>
            <CheckboxWithLabel checked={column.getIsVisible()} onCheckedChange={() => column.toggleVisibility()} />
            {column.id}
          </div>
        ))}
      </div>
      <DataTable<string | number> columns={columns} data={testData} onTableApiSet={handleTableApiSet} />
    </>
  );
};

const ExampleTable2 = () => {
  return <DataTable<string | number> columns={columns} data={testData2} />;
};
