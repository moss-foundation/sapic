import { cn } from "@/utils";
import { useGetLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/workbench/domains/layout";

export const ActivityBarButtonIndicator = () => {
  const { data: layout } = useGetLayout();

  //TODO later we should handle the JsonValue differently
  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;
  const sideBarPosition = layout?.sidebarState.position || SIDEBAR_POSITION.LEFT;

  return (
    <div
      className={cn("absolute shadow-[inset_0_-2px_10px_var(--moss-accent)] transition-[height,width] duration-300", {
        "bottom-0 left-1/2 h-0.5 w-2.5 -translate-x-1/2 rounded-t-[10px] [button:hover_+_&]:w-full":
          activityBarPosition === ACTIVITYBAR_POSITION.TOP,
        "left-1/2 top-0 h-0.5 w-2.5 -translate-x-1/2 rounded-b-[10px] [button:hover_+_&]:w-full":
          activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM,
        "top-1/2 h-2.5 w-0.5 -translate-y-1/2 [button:hover_+_&]:h-full":
          activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
        "right-0 rounded-l-[10px]":
          sideBarPosition === SIDEBAR_POSITION.RIGHT && activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
        "left-0 rounded-r-[10px]":
          sideBarPosition === SIDEBAR_POSITION.LEFT && activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT,
      })}
    />
  );
};
