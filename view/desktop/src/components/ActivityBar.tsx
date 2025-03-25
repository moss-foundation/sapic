import { HTMLAttributes } from "react";

import { Icon } from ".";
import { useActivityBarStore } from "../store/activityBarStore";
import { ActivityBarPosition, usePositionStore } from "../store/positionStore";
import { useSideBarStore } from "../store/sideBarStore";
import { cn } from "../utils";

type IconName = Parameters<typeof Icon>[0]["icon"];

interface ActivityBarProps {
  position?: ActivityBarPosition;
  activeId: number;
  onSelect: (id: number) => void;
  className?: string;
  // Spread remaining div props
  divProps?: Omit<HTMLAttributes<HTMLDivElement>, "className">;
}

const items: Array<{ id: number; iconName: IconName; label: string }> = [
  {
    id: 1,
    iconName: "CollectionsIcon",
    label: "Collections",
  },
  {
    id: 2,
    iconName: "EnvironmentsIcon",
    label: "Environments",
  },
  {
    id: 3,
    iconName: "TempIcon",
    label: "Mock",
  },
];

export function ActivityBar({ position = "left", activeId, onSelect, className, divProps }: ActivityBarProps) {
  const { sideBarPosition } = useSideBarStore();

  console.log("--------------->sideBarPosition", sideBarPosition);

  // Align ActivityBar with SideBar when position is 'default'
  const effectivePosition = position === "default" ? sideBarPosition : position;

  if (effectivePosition === "hidden") return null;

  const isVertical = effectivePosition === "left" || effectivePosition === "right";

  return (
    <div
      className={cn(
        "background-(--moss-tabslist-background) flex",
        {
          "h-full w-[48px] flex-col": isVertical,
          "h-[48px] w-full flex-row": !isVertical,
          "order-first": effectivePosition === "left",
          "order-last": effectivePosition === "right",
        },
        className
      )}
      {...divProps}
    >
      {items.map((item) => (
        <button
          key={item.id}
          onClick={() => onSelect(item.id)}
          className={cn(
            "group flex h-12 w-12 items-center justify-center",
            "hover:background-(--moss-hover-inactive-tab-background)",
            {
              "background-(--moss-active-tab-background)": activeId === item.id,
              "border-l-2 border-l-(--moss-primary)": activeId === item.id && effectivePosition === "left",
              "border-r-2 border-r-(--moss-primary)": activeId === item.id && effectivePosition === "right",
              "border-b-2 border-b-(--moss-primary)": activeId === item.id && effectivePosition === "bottom",
            }
          )}
          title={item.label}
        >
          <Icon
            icon={item.iconName}
            className={cn("h-5 w-5", {
              "text-(--moss-active-tab-text)": activeId === item.id,
              "text-(--moss-tab-text)": activeId !== item.id,
            })}
          />
        </button>
      ))}
    </div>
  );
}

export type { ActivityBarPosition } from "../store/activityBarStore";
