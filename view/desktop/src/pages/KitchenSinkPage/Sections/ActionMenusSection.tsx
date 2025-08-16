import { ActionMenu } from "@/components";
import {
  editorContextItems,
  generateItems,
  runConfigItems,
  runOptionsItems,
  runSelectorItems,
} from "@/data/actionMenuMockData";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";

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
  );
};
