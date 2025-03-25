import { ReactNode } from "react";

import { SideBarPosition } from "../store/sideBarStore";
import { cn } from "../utils";
import { ActivityBar, ActivityBarPosition } from "./ActivityBar";
import { ResizablePanel } from "./Resizable";

interface SideBarProps {
  position: SideBarPosition;
  activityBarPosition: ActivityBarPosition | "top" | "bottom";
  activeId: number;
  onSelect: (id: number) => void;
  children: ReactNode;
}

export function SideBar({
  position = "left",
  activityBarPosition = "left",
  activeId,
  onSelect,
  children,
}: SideBarProps) {
  return (
    <ResizablePanel
      preferredSize={270}
      minSize={150}
      maxSize={400}
      snap
      className={cn("flex h-full", {
        "flex-col": activityBarPosition === "top" || activityBarPosition === "bottom",
        "flex-row": activityBarPosition === "left" || activityBarPosition === "right",
        "order-first": position === "left",
        "order-last": position === "right",
      })}
    >
      {(activityBarPosition === "top" || activityBarPosition === "left") && (
        <ActivityBar
          position={activityBarPosition}
          activeId={activeId}
          onSelect={onSelect}
          className={cn({
            "h-full w-[48px]": activityBarPosition === "left",
            "h-[48px] w-full": activityBarPosition === "top",
          })}
        />
      )}
      <div className="flex-1">{children}</div>
      {(activityBarPosition === "bottom" || activityBarPosition === "right") && (
        <ActivityBar
          position={activityBarPosition}
          activeId={activeId}
          onSelect={onSelect}
          className={cn({
            "h-full w-[48px]": activityBarPosition === "right",
            "h-[48px] w-full": activityBarPosition === "bottom",
          })}
        />
      )}
    </ResizablePanel>
  );
}
