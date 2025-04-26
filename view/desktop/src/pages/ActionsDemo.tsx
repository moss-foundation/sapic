import React, { useRef, useState } from "react";
import { ActionMenu, MenuItemProps } from "@/components/ActionMenu/ActionMenu";

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

  // 1. Editor context menu items
  const editorContextItems: MenuItemProps[] = [
    { id: "header-1", type: "header", label: "Show Context Actions" },
    { id: "build", type: "action", label: "Build", shortcut: "⌘V", icon: "TestBuild" },
    {
      id: "copy-paste",
      type: "submenu",
      label: "Copy / Paste Special",
      items: [
        { id: "copy-as-html", type: "action", label: "Copy as HTML" },
        { id: "paste-from-history", type: "action", label: "Paste from History" },
      ],
    },
    { id: "column-selections", type: "action", label: "Column Selections Mode", shortcut: "⌥8" },
    { id: "separator-1", type: "separator" },
    { id: "find-usages", type: "action", label: "Find Usages" },
    {
      id: "go-to",
      type: "submenu",
      label: "Go To",
      items: [
        { id: "go-to-line", type: "action", label: "Line/Column..." },
        { id: "go-to-file", type: "action", label: "File..." },
      ],
    },
    {
      id: "folding",
      type: "submenu",
      label: "Folding",
      items: [
        { id: "expand-all", type: "action", label: "Expand All" },
        { id: "collapse-all", type: "action", label: "Collapse All" },
      ],
    },
    {
      id: "analyze",
      type: "submenu",
      label: "Analyze",
      items: [
        { id: "inspect-code", type: "action", label: "Inspect Code..." },
        { id: "code-cleanup", type: "action", label: "Code Cleanup..." },
      ],
    },
    {
      id: "refactor",
      type: "submenu",
      label: "Refactor",
      items: [
        { id: "rename", type: "action", label: "Rename..." },
        { id: "extract-method", type: "action", label: "Extract Method..." },
      ],
    },
    {
      id: "generate",
      type: "submenu",
      label: "Generate...",
      shortcut: "⌘N",
      items: [
        { id: "test-method", type: "action", label: "Test Method" },
        { id: "setup-method", type: "action", label: "SetUp Method" },
        { id: "teardown-method", type: "action", label: "TearDown Method" },
        { id: "separator-gen-1", type: "separator" },
        { id: "constructor", type: "action", label: "Constructor" },
        { id: "getter", type: "action", label: "Getter" },
        { id: "setter", type: "action", label: "Setter" },
        { id: "getter-setter", type: "action", label: "Getter and Setter" },
        { id: "equals-hashCode", type: "action", label: "equals() and hashCode()" },
        { id: "toString", type: "action", label: "toString()" },
      ],
    },
    { id: "separator-2", type: "separator" },
    { id: "run", type: "action", label: "Run 'PetClinicApplication'", icon: "TestRun", iconColor: "green" },
    { id: "debug", type: "action", label: "Debug 'PetClinicApplication'", icon: "TestDebug" },
    {
      id: "more-run-debug",
      type: "submenu",
      label: "More Run/Debug",
      items: [
        { id: "run-with-coverage", type: "action", label: "Run with Coverage", icon: "TestRunWith" },
        { id: "profile", type: "action", label: "Profile", icon: "TestProfileWith" },
      ],
    },
    { id: "separator-3", type: "separator" },
    { id: "open-split", type: "action", label: "Open in Split with Chooser...", shortcut: "⌥⌘," },
    {
      id: "open-in",
      type: "submenu",
      label: "Open In",
      items: [
        { id: "open-in-new-window", type: "action", label: "New Window" },
        { id: "open-in-browser", type: "action", label: "Browser" },
      ],
    },
    { id: "separator-4", type: "separator" },
    { id: "local-history", type: "action", label: "Local History" },
    {
      id: "git",
      type: "submenu",
      label: "Git",
      items: [
        { id: "git-add", type: "action", label: "Add" },
        { id: "git-commit", type: "action", label: "Commit..." },
      ],
    },
    { id: "separator-5", type: "separator" },
    { id: "compare-clipboard", type: "action", label: "Compare with Clipboard", icon: "TestCompare" },
    {
      id: "diagrams",
      type: "submenu",
      label: "Diagrams",
      items: [
        { id: "show-diagram", type: "action", label: "Show Diagram" },
        { id: "analyze-deps", type: "action", label: "Analyze Dependencies..." },
      ],
    },
  ];

  // 2. Generate menu items (focused view of the generate submenu)
  const generateItems: MenuItemProps[] = [
    { id: "header-2", type: "header", label: "Generate" },
    {
      id: "test-method",
      type: "submenu",
      label: "Test Method",
      items: [
        { id: "junit-test", type: "action", label: "JUnit Test" },
        { id: "testng-test", type: "action", label: "TestNG Test" },
      ],
    },
    {
      id: "setup-method",
      type: "submenu",
      label: "SetUp Method",
      items: [
        { id: "junit-setup", type: "action", label: "JUnit SetUp" },
        { id: "testng-setup", type: "action", label: "TestNG SetUp" },
      ],
    },
    {
      id: "teardown-method",
      type: "submenu",
      label: "TearDown Method",
      items: [
        { id: "junit-teardown", type: "action", label: "JUnit TearDown" },
        { id: "testng-teardown", type: "action", label: "TestNG TearDown" },
      ],
    },
    { id: "separator-gen-1", type: "separator" },
    { id: "constructor", type: "action", label: "Constructor" },
    { id: "getter", type: "action", label: "Getter" },
    { id: "setter", type: "action", label: "Setter" },
    { id: "getter-setter", type: "action", label: "Getter and Setter" },
    { id: "equals-hashCode", type: "action", label: "equals() and hashCode()" },
    { id: "toString", type: "action", label: "toString()" },
  ];

  // 3. Run configuration selector menu
  const runConfigItems: MenuItemProps[] = [
    { id: "header-3", type: "section", sectionTitle: "Recent" },
    { id: "accuratemath-app", type: "action", label: "AccurateMath app", icon: "Folder" },
    { id: "server", type: "action", label: "Server", icon: "Folder" },
    { id: "app-tests", type: "action", label: "App tests", icon: "TestTests" },
    { id: "server-tests", type: "action", label: "Server tests", icon: "TestTests" },
    { id: "separator-run-1", type: "separator" },
    { id: "all-configs", type: "action", label: "All Configurations", count: 25 },
    { id: "edit-configs", type: "action", label: "Edit Configurations...", shortcut: "⌃E" },
  ];

  // 4. Run options menu
  const runOptionsItems: MenuItemProps[] = [
    { id: "profile-intellij", type: "action", label: "Profile with 'IntelliJ Profiler'", icon: "TestProfileWith" },
    { id: "run-with-coverage", type: "action", label: "Run with Coverage", icon: "TestRunWith" },
    { id: "separator-options-1", type: "separator" },
    { id: "section-config", type: "section", sectionTitle: "Configuration" },
    { id: "edit-config", type: "action", label: "Edit..." },
    { id: "delete-config", type: "action", label: "Delete..." },
    { id: "remove-recent", type: "action", label: "Remove from Recent" },
  ];

  // 5. Run selector menu
  const runSelectorItems: MenuItemProps[] = [
    { id: "header-run", type: "header", label: "Run" },
    { id: "edit-configurations", type: "action", label: "Edit Configurations..." },
    { id: "separator-run-sel-1", type: "separator" },
    { id: "idea", type: "action", label: "IDEA", icon: "Folder" },
    { id: "idea-android", type: "action", label: "IDEA with Android", icon: "Folder" },
    { id: "idea-python", type: "action", label: "IDEA with Python plugin", icon: "Folder" },
    { id: "pycharm", type: "action", label: "PyCharm", icon: "Folder" },
    { id: "dart-tests", type: "action", label: "Dart tests", icon: "TestTests" },
    { id: "generate-icon-classes", type: "action", label: "Generate icon classes", icon: "TestTests" },
    { id: "footer-1", type: "footer", footerText: "Hold ⌥ to debug" },
  ];

  // 6. Theme selector dropdown
  const themeItems: MenuItemProps[] = [
    { id: "light", type: "radio", label: "Light", value: "light" },
    { id: "dark", type: "radio", label: "Dark", value: "dark" },
    { id: "separator-theme-1", type: "separator" },
    { id: "high-contrast", type: "radio", label: "High Contrast", value: "high-contrast" },
    { id: "darcula", type: "radio", label: "Darcula", value: "darcula" },
    { id: "intellij-light", type: "radio", label: "IntelliJ Light", value: "intellij-light" },
  ];

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
