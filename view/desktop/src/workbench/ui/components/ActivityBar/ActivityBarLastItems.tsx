import { ACTIVITYBAR_POSITION } from "@/constants/layout";
import { useDescribeApp } from "@/hooks";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

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
      <ActivityBarButton
        icon="Person"
        id="1"
        title="Profile"
        order={1}
        onClick={() =>
          addOrFocusPanel({
            id: "Profile",
            component: "Profile",
          })
        }
        isDraggable={false}
      />
      <ActivityBarButton
        icon="Settings"
        id="2"
        title="Settings"
        order={2}
        onClick={() =>
          addOrFocusPanel({
            id: "Settings",
            component: "Settings",
          })
        }
        isDraggable={false}
      />
    </div>
  );
};
