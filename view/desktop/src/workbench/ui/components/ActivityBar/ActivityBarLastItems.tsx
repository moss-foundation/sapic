import { useDescribeApp } from "@/hooks";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { ACTIVITYBAR_POSITION } from "@/workbench/domains/layout";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ActionMenu } from "..";
import { ActivityBarButton } from "./ActivityBarButton";

export const ActivityBarLastItems = () => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const { data: appState } = useDescribeApp();
  const activityBarPosition = appState?.configuration.contents.activityBarPosition || ACTIVITYBAR_POSITION.DEFAULT;

  return (
    <div
      className={cn("flex", {
        "flex-col gap-3": activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
        "flex-row gap-1":
          activityBarPosition === ACTIVITYBAR_POSITION.TOP || activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM,
      })}
    >
      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <ActivityBarButton icon="Preferences" id="1" title="Preferences" order={1} isDraggable={false} />
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
