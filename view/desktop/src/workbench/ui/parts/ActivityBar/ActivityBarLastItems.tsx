import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION } from "@/workbench/domains/layout";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu } from "@/workbench/ui/components";
import { ActivityBarButton } from "./ActivityBarButton";

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
        icon="Puzzle"
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
          <ActivityBarButton icon="Preferences" id="preferences" title="Preferences" order={2} isDraggable={false} />
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
