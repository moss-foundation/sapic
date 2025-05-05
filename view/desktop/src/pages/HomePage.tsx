import React from "react";
import { useTranslation } from "react-i18next";

import { ActionMenu, MenuItemProps } from "@/components/ActionMenu/ActionMenu";
import {
  editorContextItems,
  generateItems,
  runConfigItems,
  runOptionsItems,
  runSelectorItems,
  themeItems,
} from "@/data/actionMenuMockData";
import { invokeMossCommand } from "@/lib/backend/platfrom.ts";

import * as iconsNames from "../assets/icons";
import { Button, Icon, Icons, Scrollbar } from "../components";

export const Home = () => {
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
  const [contextMenuOpen, setContextMenuOpen] = React.useState(false);
  const [generateMenuOpen, setGenerateMenuOpen] = React.useState(false);
  const [runConfigMenuOpen, setRunConfigMenuOpen] = React.useState(false);
  const [runOptionsMenuOpen, setRunOptionsMenuOpen] = React.useState(false);
  const [runSelectorMenuOpen, setRunSelectorMenuOpen] = React.useState(false);
  const [themeMenuOpen, setThemeMenuOpen] = React.useState(false);
  const [selectedTheme, setSelectedTheme] = React.useState("light");

  React.useEffect(() => {
    const fetchData = async () => {
      setData(Math.floor(Math.random() * 100));
    };

    fetchData();
  }, []);

  const intent = ["primary", "neutral"];
  const variant = ["solid", "outlined"];

  const handleItemSelect = (item: MenuItemProps) => {
    console.log(`Selected: ${item.id}`);

    // Handle theme selection
    if (item.type === "radio" && item.value) {
      setSelectedTheme(item.value);
    }
  };

  console.log(Object.keys(iconsNames));

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
              <ActionMenu
                items={editorContextItems}
                open={contextMenuOpen}
                onOpenChange={setContextMenuOpen}
                onSelect={handleItemSelect}
                align="start"
                trigger={
                  <button
                    className="w-fit rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                    onClick={() => setContextMenuOpen(true)}
                  >
                    Show Context Actions
                  </button>
                }
              />

              {/* Generate Menu Button */}
              <ActionMenu
                items={generateItems}
                open={generateMenuOpen}
                onOpenChange={setGenerateMenuOpen}
                onSelect={handleItemSelect}
                align="start"
                trigger={
                  <button
                    className="w-fit rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                    onClick={() => setGenerateMenuOpen(true)}
                  >
                    Generate Menu
                  </button>
                }
              />

              {/* Run Configurations Button */}
              <ActionMenu
                items={runConfigItems}
                open={runConfigMenuOpen}
                onOpenChange={setRunConfigMenuOpen}
                onSelect={handleItemSelect}
                align="start"
                trigger={
                  <button
                    className="w-fit rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                    onClick={() => setRunConfigMenuOpen(true)}
                  >
                    Run Configurations
                  </button>
                }
              />

              {/* Run Options Button */}
              <ActionMenu
                items={runOptionsItems}
                open={runOptionsMenuOpen}
                onOpenChange={setRunOptionsMenuOpen}
                onSelect={handleItemSelect}
                align="start"
                trigger={
                  <button
                    className="w-fit rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                    onClick={() => setRunOptionsMenuOpen(true)}
                  >
                    Run Options
                  </button>
                }
              />

              {/* Run Selector Button */}
              <ActionMenu
                items={runSelectorItems}
                open={runSelectorMenuOpen}
                onOpenChange={setRunSelectorMenuOpen}
                onSelect={handleItemSelect}
                align="start"
                trigger={
                  <button
                    className="w-fit rounded-md bg-gray-200 px-4 py-2 font-medium text-gray-800 shadow transition-colors hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                    onClick={() => setRunSelectorMenuOpen(true)}
                  >
                    Run Selector
                  </button>
                }
              />
            </div>
          </div>

          {/* Theme Selector Dropdown */}
          <div>
            <h3 className="mb-4 text-xl font-medium text-gray-700 dark:text-gray-200">Dropdown Menu</h3>
            <div className="flex items-center gap-3 rounded-md bg-gray-100 p-4 dark:bg-gray-800/50">
              <span className="font-medium text-gray-700 dark:text-gray-300">Theme:</span>
              <div className="w-56">
                <ActionMenu
                  type="dropdown"
                  items={themeItems}
                  open={themeMenuOpen}
                  onOpenChange={setThemeMenuOpen}
                  onSelect={handleItemSelect}
                  selectedValue={selectedTheme}
                  placeholder="Select theme"
                />
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Button Components */}
      <section className="rounded-xl bg-white p-6 shadow-md dark:bg-stone-800">
        <h2 className="mb-4 text-2xl font-bold text-gray-800 dark:text-gray-100">Button Components</h2>
        <p className="mb-6 text-gray-600 dark:text-gray-300">
          Various button states and variants available in the application.
        </p>

        <div className="mb-10 space-y-8">
          <div>
            <h3 className="mb-4 text-xl font-medium text-gray-700 dark:text-gray-200">Standard Buttons</h3>
            <div className="overflow-hidden rounded-lg border border-gray-200 dark:border-gray-700">
              <table className="w-full border-collapse">
                <thead>
                  <tr className="bg-gray-100 dark:bg-gray-700">
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Variant</th>
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Primary</th>
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Neutral</th>
                  </tr>
                </thead>
                <tbody>
                  {variant.map((v) => (
                    <tr key={v} className="border-t border-gray-200 dark:border-gray-700">
                      <td className="p-3 font-medium text-gray-700 dark:text-gray-300">{v}</td>
                      {intent.map((i) => (
                        <td key={`${v}-${i}`} className="p-3">
                          <Button variant={v as any} intent={i as any} size="md" loading={false}>
                            OK
                          </Button>
                        </td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          <div>
            <h3 className="mb-4 text-xl font-medium text-gray-700 dark:text-gray-200">Disabled Buttons</h3>
            <div className="overflow-hidden rounded-lg border border-gray-200 dark:border-gray-700">
              <table className="w-full border-collapse">
                <thead>
                  <tr className="bg-gray-100 dark:bg-gray-700">
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Variant</th>
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Primary</th>
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Neutral</th>
                  </tr>
                </thead>
                <tbody>
                  {variant.map((v) => (
                    <tr key={v} className="border-t border-gray-200 dark:border-gray-700">
                      <td className="p-3 font-medium text-gray-700 dark:text-gray-300">{v}</td>
                      {intent.map((i) => (
                        <td key={`${v}-${i}`} className="p-3">
                          <Button disabled variant={v as any} intent={i as any} size="md">
                            {t("button")}
                          </Button>
                        </td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          <div>
            <h3 className="mb-4 text-xl font-medium text-gray-700 dark:text-gray-200">Loading Buttons</h3>
            <div className="overflow-hidden rounded-lg border border-gray-200 dark:border-gray-700">
              <table className="w-full border-collapse">
                <thead>
                  <tr className="bg-gray-100 dark:bg-gray-700">
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Variant</th>
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Primary</th>
                    <th className="p-3 text-left font-medium text-gray-600 dark:text-gray-300">Neutral</th>
                  </tr>
                </thead>
                <tbody>
                  {variant.map((v) => (
                    <tr key={v} className="border-t border-gray-200 dark:border-gray-700">
                      <td className="p-3 font-medium text-gray-700 dark:text-gray-300">{v}</td>
                      {intent.map((i) => (
                        <td key={`${v}-${i}`} className="p-3">
                          <Button variant={v as any} intent={i as any} size="md" loading>
                            {t("button")}
                          </Button>
                        </td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
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
