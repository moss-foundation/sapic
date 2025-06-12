import { ACTIVITYBAR_POSITION } from "@/constants/layoutPositions";
import { useActivityBarStore } from "@/store/activityBar";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

import { ActivityBarButton } from "./ActivityBarButton";

export const ActivityBarLastItems = () => {
  const { openPanel } = useTabbedPaneStore();
  const { position } = useActivityBarStore();
  return (
    <div
      className={cn("flex", {
        "flex-col gap-3": position === ACTIVITYBAR_POSITION.DEFAULT,
        "flex-row gap-1": position === ACTIVITYBAR_POSITION.TOP || position === ACTIVITYBAR_POSITION.BOTTOM,
      })}
    >
      <ActivityBarButton
        icon="Person"
        iconActive="PersonActive"
        isActive={false}
        id="1"
        title="User"
        order={1}
        onClick={() => openPanel("Settings")}
      />
      <ActivityBarButton
        icon="Settings"
        iconActive="SettingsActive"
        isActive={false}
        id="2"
        title="Settings"
        order={2}
        onClick={() => openPanel("Settings")}
      />
    </div>
  );
};
