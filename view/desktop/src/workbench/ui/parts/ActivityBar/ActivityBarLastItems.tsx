import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION } from "@/workbench/domains/layout";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu } from "@/workbench/ui/components";

import { ActivityBarButton } from "./components/ActivityBarButton";

export const ActivityBarLastItems = () => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const { data: layout } = useGetLayout();
  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;

  return (
    <div
      className={cn("flex", {
        "flex-col gap-3": activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
        "flex-row gap-1":
          activityBarPosition === ACTIVITYBAR_POSITION.TOP || activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM,
      })}
    >
      <ActivityBarButton
        icon={
          <svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
              d="M5.125 2.875C5.125 1.83947 5.96447 1 7 1C8.03552 1 8.875 1.83947 8.875 2.875V4H9.625C10.6733 4 11.1975 4 11.6111 4.17127C12.1623 4.39963 12.6004 4.83765 12.8287 5.38896C13 5.80245 13 6.32664 13 7.375H14.125C15.1605 7.375 16 8.21448 16 9.25C16 10.2855 15.1605 11.125 14.125 11.125H13V12.4C13 13.6602 13 14.2901 12.7548 14.7715C12.5391 15.1949 12.1949 15.5391 11.7715 15.7548C11.2901 16 10.6601 16 9.4 16H8.875V14.6875C8.875 13.7556 8.11945 13 7.1875 13C6.25552 13 5.5 13.7556 5.5 14.6875V16H4.6C3.33988 16 2.70982 16 2.22852 15.7548C1.80516 15.5391 1.46095 15.1949 1.24524 14.7715C1 14.2901 1 13.6602 1 12.4V11.125H2.125C3.16053 11.125 4 10.2855 4 9.25C4 8.21448 3.16053 7.375 2.125 7.375H1C1 6.32664 1 5.80245 1.17127 5.38896C1.39963 4.83765 1.83765 4.39963 2.38896 4.17127C2.80244 4 3.32664 4 4.375 4H5.125V2.875Z"
              stroke="#383A42"
              strokeWidth="1.2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        }
        id="extensions"
        title="Extensions"
        order={1}
        isDraggable={false}
        onClick={() => {
          addOrFocusPanel({
            id: "Extensions",
            component: "DefaultView",
            title: "Extensions",
            params: {
              tabIcon: "Puzzle",
            },
          });
        }}
      />

      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <ActivityBarButton
            icon={
              <svg width="18" height="18" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path
                  d="M2 6.25H11M11 6.25C11 7.49264 12.0073 8.5 13.25 8.5C14.4927 8.5 15.5 7.49264 15.5 6.25C15.5 5.00736 14.4927 4 13.25 4C12.0073 4 11 5.00736 11 6.25ZM6.5 12.25H15.5M6.5 12.25C6.5 13.4927 5.49264 14.5 4.25 14.5C3.00736 14.5 2 13.4927 2 12.25C2 11.0073 3.00736 10 4.25 10C5.49264 10 6.5 11.0073 6.5 12.25Z"
                  stroke="#383A42"
                  strokeWidth="1.2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
              </svg>
            }
            id="preferences"
            title="Preferences"
            order={2}
            isDraggable={false}
          />
        </ActionMenu.Trigger>
        <ActionMenu.Content>
          <ActionMenu.Item>
            <button
              className="flex w-full cursor-pointer items-center gap-2"
              onClick={() => {
                addOrFocusPanel({
                  id: "Settings",
                  component: "SettingsView",
                });
              }}
            >
              <Icon icon="Settings" className="size-4.5" />
              <span>Settings</span>
            </button>
          </ActionMenu.Item>
          <ActionMenu.Item>
            <button
              className="flex w-full cursor-pointer items-center gap-2"
              onClick={() => {
                addOrFocusPanel({
                  id: "Accounts",
                  component: "AccountsView",
                  params: {
                    tabIcon: "Accounts",
                  },
                });
              }}
            >
              <Icon icon="Accounts" className="size-4.5" />
              <span>Accounts</span>
            </button>
          </ActionMenu.Item>
        </ActionMenu.Content>
      </ActionMenu.Root>
    </div>
  );
};
