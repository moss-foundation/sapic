import { useRef } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { DropIndicator } from "@/workbench/ui/components";

import { useDraggableStatusBarButton } from "../dnd/hooks/useDraggableStatusBarButton";
import { StatusBarItem } from "../types";

interface StatusBarDraggableButtonProps {
  statusBarItem: StatusBarItem;
}

export const StatusBarDraggableButton = ({ statusBarItem }: StatusBarDraggableButtonProps) => {
  const ref = useRef<HTMLButtonElement | null>(null);
  const { closestEdge } = useDraggableStatusBarButton({ ref, statusBarItem });

  return (
    <button ref={ref} className={cn("relative flex h-full items-center justify-center")}>
      <div className="hover:background-(--moss-secondary-background-hover) text-(--moss-primary-foreground) flex items-center gap-1 rounded py-[3px] pl-1.5 pr-1 transition">
        {statusBarItem.icon && <Icon icon={statusBarItem.icon} className="size-3.5" />}
        {statusBarItem.label && <span className="">{statusBarItem.label}</span>}
      </div>
      {closestEdge ? <DropIndicator edge={closestEdge} gap={4} /> : null}
    </button>
  );
};
