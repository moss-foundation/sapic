import React, { useState } from "react";
import { ActionMenu, MenuItemProps } from "@/components/ActionMenu/ActionMenu";
import {
  editorContextItems,
  generateItems,
  runConfigItems,
  runOptionsItems,
  runSelectorItems,
  themeItems,
} from "@/data/actionMenuMockData";

const ActionsDemo: React.FC = () => {
  // State for the different menus
  const [contextMenuOpen, setContextMenuOpen] = useState(false);
  const [generateMenuOpen, setGenerateMenuOpen] = useState(false);
  const [runConfigMenuOpen, setRunConfigMenuOpen] = useState(false);
  const [runOptionsMenuOpen, setRunOptionsMenuOpen] = useState(false);
  const [runSelectorMenuOpen, setRunSelectorMenuOpen] = useState(false);
  const [themeMenuOpen, setThemeMenuOpen] = useState(false);

  // State for selected values
  const [selectedTheme, setSelectedTheme] = useState("light");

  const handleItemSelect = (item: MenuItemProps) => {
    console.log(`Selected: ${item.id}`);

    // Handle theme selection
    if (item.type === "radio" && item.value) {
      setSelectedTheme(item.value);
    }
  };

  return (
    <div className="flex h-full flex-col p-5">
      <h1 className="mb-3 text-2xl font-bold">Action Menu Demo</h1>

      <div className="mb-10 flex flex-wrap gap-4">
        {/* Context Actions Button */}
        <ActionMenu
          items={editorContextItems}
          open={contextMenuOpen}
          onOpenChange={setContextMenuOpen}
          width={293}
          maxHeight={553}
          menuItemHeight={24}
          onSelect={handleItemSelect}
          align="start"
          trigger={
            <button
              className="w-fit rounded-md bg-(--moss-secondary-background) px-4 py-2 shadow transition-colors hover:bg-(--moss-secondary-background-hover)"
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
          width={240}
          onSelect={handleItemSelect}
          align="start"
          trigger={
            <button
              className="w-fit rounded-md bg-(--moss-secondary-background) px-4 py-2 shadow transition-colors hover:bg-(--moss-secondary-background-hover)"
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
          width={260}
          onSelect={handleItemSelect}
          align="start"
          trigger={
            <button
              className="w-fit rounded-md bg-(--moss-secondary-background) px-4 py-2 shadow transition-colors hover:bg-(--moss-secondary-background-hover)"
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
          width={250}
          onSelect={handleItemSelect}
          align="start"
          trigger={
            <button
              className="w-fit rounded-md bg-(--moss-secondary-background) px-4 py-2 shadow transition-colors hover:bg-(--moss-secondary-background-hover)"
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
          width={260}
          onSelect={handleItemSelect}
          align="start"
          trigger={
            <button
              className="w-fit rounded-md bg-(--moss-secondary-background) px-4 py-2 shadow transition-colors hover:bg-(--moss-secondary-background-hover)"
              onClick={() => setRunSelectorMenuOpen(true)}
            >
              Run Selector
            </button>
          }
        />
      </div>

      {/* Theme Selector Dropdown */}
      <div className="mb-10">
        <div className="flex items-center gap-2">
          <span className="font-medium">Theme:</span>
          <div className="w-40">
            <ActionMenu
              type="dropdown"
              items={themeItems}
              open={themeMenuOpen}
              onOpenChange={setThemeMenuOpen}
              width={220}
              onSelect={handleItemSelect}
              selectedValue={selectedTheme}
              placeholder="Select theme"
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default ActionsDemo;
