import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

export const ActivityBarButtonIndicator = () => {
  const position = useActivityBarStore((state) => state.position);
  const sideBarPosition = useAppResizableLayoutStore((state) => state.sideBarPosition);

  return (
    <div
      className={cn("absolute shadow-[inset_0_-2px_10px_var(--moss-accent)] transition-[height,width] duration-300", {
        "bottom-0 left-1/2 h-0.5 w-2.5 -translate-x-1/2 rounded-t-[10px] [button:hover_+_&]:w-full":
          position === ACTIVITYBAR_POSITION.TOP,
        "left-1/2 top-0 h-0.5 w-2.5 -translate-x-1/2 rounded-b-[10px] [button:hover_+_&]:w-full":
          position === ACTIVITYBAR_POSITION.BOTTOM,
        "top-1/2 h-2.5 w-0.5 -translate-y-1/2 [button:hover_+_&]:h-full": position === ACTIVITYBAR_POSITION.DEFAULT,
        "right-0 rounded-l-[10px]":
          sideBarPosition === SIDEBAR_POSITION.RIGHT && position === ACTIVITYBAR_POSITION.DEFAULT,
        "left-0 rounded-r-[10px]":
          sideBarPosition === SIDEBAR_POSITION.LEFT && position === ACTIVITYBAR_POSITION.DEFAULT,
      })}
    />
  );
};
