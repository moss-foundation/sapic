import { Icons } from "@/lib/ui";
import { ActionMenu } from "@/workbench/ui/components";
import { MenuItemProps, renderActionMenuItem } from "@/workbench/utils/renderActionMenuItem";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const ActionMenusSection = () => {
  return (
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
              <ActionMenu.Content>
                {generateItems.map((item) => renderActionMenuItem(item))}

                <ActionMenu.Footer>
                  <span>Footer</span>
                </ActionMenu.Footer>
              </ActionMenu.Content>
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
  );
};

// 1. Editor context menu items
export const editorContextItems: MenuItemProps[] = [
  {
    id: "show-context-actions",
    type: "action",
    label: "Show Context Actions",
    shortcut: "⌘⏎",
    icon: "Placeholder" as Icons,
  },
  { id: "separator-1", type: "separator" },
  { id: "build", type: "action", label: "Build", shortcut: "⌘V", icon: "Placeholder" },
  {
    id: "copy-paste",
    type: "submenu",
    label: "Copy / Paste Special",
    alignWithIcons: true,
    items: [
      { id: "copy-as-html", type: "action", label: "Copy as HTML" },
      { id: "paste-from-history", type: "action", label: "Paste from History" },
    ],
  },
  { id: "column-selections", type: "action", label: "Column Selections Mode", shortcut: "⇧⌘8", alignWithIcons: true },
  { id: "separator-2", type: "separator" },
  { id: "find-usages", type: "action", label: "Find Usages", alignWithIcons: true },
  { id: "separator-3", type: "separator" },
  {
    id: "go-to",
    type: "submenu",
    label: "Go To",
    alignWithIcons: true,
    items: [
      { id: "go-to-line", type: "action", label: "Line/Column..." },
      { id: "go-to-file", type: "action", label: "File..." },
    ],
  },
  {
    id: "folding",
    type: "submenu",
    label: "Folding",
    alignWithIcons: true,
    items: [
      { id: "expand-all", type: "action", label: "Expand All" },
      { id: "collapse-all", type: "action", label: "Collapse All" },
    ],
  },
  {
    id: "analyze",
    type: "submenu",
    label: "Analyze",
    alignWithIcons: true,
    items: [
      { id: "inspect-code", type: "action", label: "Inspect Code..." },
      { id: "code-cleanup", type: "action", label: "Code Cleanup..." },
    ],
  },
  { id: "separator-4", type: "separator" },
  {
    id: "refactor",
    type: "submenu",
    label: "Refactor",
    alignWithIcons: true,
    items: [
      { id: "rename", type: "action", label: "Rename..." },
      { id: "extract-method", type: "action", label: "Extract Method..." },
    ],
  },
  {
    id: "generate",
    type: "action",
    label: "Generate...",
    shortcut: "⌘N",
    alignWithIcons: true,
  },
  { id: "separator-5", type: "separator" },
  { id: "run", type: "action", label: "Run 'PetClinicApplication'", icon: "Placeholder", iconColor: "green" },
  { id: "debug", type: "action", label: "Debug 'PetClinicApplication'", icon: "Placeholder" },
  {
    id: "more-run-debug",
    type: "submenu",
    label: "More Run/Debug",
    alignWithIcons: true,
    items: [
      { id: "run-with-coverage", type: "action", label: "Run with Coverage", icon: "Placeholder" },
      { id: "profile", type: "action", label: "Profile", icon: "Placeholder" },
    ],
  },
  { id: "separator-6", type: "separator" },
  { id: "open-split", type: "action", label: "Open in Split with Chooser...", shortcut: "⌥⇧⏎,", alignWithIcons: true },
  {
    id: "open-in",
    type: "submenu",
    label: "Open In",
    alignWithIcons: true,
    items: [
      { id: "open-in-new-window", type: "action", label: "New Window" },
      { id: "open-in-browser", type: "action", label: "Browser" },
    ],
  },
  { id: "separator-7", type: "separator" },
  {
    id: "local-history",
    type: "submenu",
    label: "Local History",
    alignWithIcons: true,
    items: [
      { id: "show-history", type: "action", label: "Show History" },
      { id: "put-label", type: "action", label: "Put Label..." },
      { id: "separator-history-1", type: "separator" },
      { id: "revert", type: "action", label: "Revert..." },
    ],
  },
  {
    id: "git",
    type: "submenu",
    label: "Git",
    alignWithIcons: true,
    items: [
      { id: "git-add", type: "action", label: "Add" },
      { id: "git-commit", type: "action", label: "Commit..." },
    ],
  },
  { id: "separator-8", type: "separator" },
  { id: "compare-clipboard", type: "action", label: "Compare with Clipboard", icon: "Placeholder" },
  { id: "separator-9", type: "separator" },
  {
    id: "diagrams",
    type: "submenu",
    label: "Diagrams",
    alignWithIcons: true,
    items: [
      { id: "show-diagram", type: "action", label: "Show Diagram" },
      { id: "analyze-deps", type: "action", label: "Analyze Dependencies..." },
    ],
  },
];

// 2. Generate menu items (focused view of the generate submenu)
export const generateItems: MenuItemProps[] = [
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
export const runConfigItems: MenuItemProps[] = [
  { id: "header-3", type: "section", sectionTitle: "Recent" },
  { id: "accuratemath-app", type: "action", label: "AccurateMath app", icon: "Placeholder" },
  { id: "server", type: "action", label: "Server", icon: "Placeholder" },
  { id: "app-tests", type: "action", label: "App tests", icon: "Placeholder" },
  { id: "server-tests", type: "action", label: "Server tests", icon: "Placeholder" },
  { id: "separator-run-1", type: "separator" },
  {
    id: "all-configs",
    type: "accordion",
    label: "All Configurations",
    icon: "ChevronRight",
    items: [
      { id: "config-web-app", type: "action", label: "Web Application", icon: "Placeholder" },
      { id: "config-mobile-app", type: "action", label: "Mobile Application", icon: "Placeholder" },
      { id: "config-api-server", type: "action", label: "API Server", icon: "Placeholder" },
      { id: "config-database", type: "action", label: "Database Integration", icon: "Placeholder" },
      { id: "config-junit", type: "action", label: "JUnit Tests", icon: "Placeholder" },
    ],
  },
  { id: "separator-run-2", type: "separator" },
  { id: "edit-configs", type: "action", label: "Edit Configurations...", shortcut: "^⌥E", alignWithIcons: true },
];

// 4. Run options menu
export const runOptionsItems: MenuItemProps[] = [
  { id: "profile-intellij", type: "action", label: "Profile with 'IntelliJ Profiler'", icon: "Placeholder" },
  { id: "run-with-coverage", type: "action", label: "Run with Coverage", icon: "Placeholder" },
  { id: "separator-options-1", type: "separator" },
  { id: "section-config", type: "section", sectionTitle: "Configuration" },
  { id: "edit-config", type: "action", label: "Edit...", alignWithIcons: true },
  { id: "delete-config", type: "action", label: "Delete...", alignWithIcons: true },
  { id: "remove-recent", type: "action", label: "Remove from Recent", alignWithIcons: true },
];

// 5. Run selector menu
export const runSelectorItems: MenuItemProps[] = [
  { id: "header-run", type: "header", label: "Run" },
  { id: "edit-configurations", type: "action", label: "Edit Configurations..." },
  { id: "separator-run-sel-1", type: "separator" },
  {
    id: "idea",
    type: "submenu",
    label: "IDEA",
    icon: "Placeholder",
    items: [
      { id: "idea-run", type: "action", label: "Run" },
      { id: "idea-debug", type: "action", label: "Debug" },
      { id: "idea-profile", type: "action", label: "Profile" },
    ],
  },
  {
    id: "idea-android",
    type: "submenu",
    label: "IDEA with Android",
    icon: "Placeholder",
    items: [
      { id: "android-run", type: "action", label: "Run on Device" },
      { id: "android-emulator", type: "action", label: "Run on Emulator" },
      { id: "android-debug", type: "action", label: "Debug" },
    ],
  },
  {
    id: "idea-python",
    type: "submenu",
    label: "IDEA with Python plugin",
    icon: "Placeholder",
    items: [
      { id: "python-run", type: "action", label: "Run" },
      { id: "python-debug", type: "action", label: "Debug" },
      { id: "python-terminal", type: "action", label: "Python Console" },
    ],
  },
  {
    id: "pycharm",
    type: "submenu",
    label: "PyCharm",
    icon: "Placeholder",
    items: [
      { id: "pycharm-run", type: "action", label: "Run" },
      { id: "pycharm-debug", type: "action", label: "Debug" },
      { id: "pycharm-coverage", type: "action", label: "Run with Coverage" },
    ],
  },
  { id: "separator-run-sel-2", type: "separator" },
  { id: "dart-tests", type: "action", label: "Dart tests", icon: "Placeholder" },
  { id: "generate-icon-classes", type: "action", label: "Generate icon classes", icon: "Placeholder" },
  { id: "footer-1", type: "footer", footerText: "Hold ⇧ to debug" },
];

// 6. Theme selector dropdown
export const themeItems: MenuItemProps[] = [
  { id: "light", type: "radio", label: "Light", value: "light" },
  { id: "dark", type: "radio", label: "Dark", value: "dark" },
  { id: "separator-theme-1", type: "separator" },
  { id: "high-contrast", type: "radio", label: "High Contrast", value: "high-contrast" },
  { id: "darcula", type: "radio", label: "Darcula", value: "darcula" },
  { id: "intellij-light", type: "radio", label: "IntelliJ Light", value: "intellij-light" },
];
