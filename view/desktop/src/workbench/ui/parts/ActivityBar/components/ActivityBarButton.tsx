import { useRef } from "react";
import { createPortal } from "react-dom";

import { useCurrentWorkspace } from "@/hooks";
import { cn } from "@/utils";
import { useGetLayout, useUpdateLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION } from "@/workbench/domains/layout";
import DropIndicator from "@/workbench/ui/components/DropIndicator";
import { ActivityBarButtonProps } from "@/workbench/ui/parts/ActivityBar/types";

import { useDraggableActivityBarButton } from "../hooks/useDraggableActivityBarButton";

export const ActivityBarButton = ({ icon, iconActive, isDraggable, ...props }: ActivityBarButtonProps) => {
  const activityButtonRef = useRef<HTMLButtonElement | null>(null);

  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;
  const isActive = props.id === layout?.activitybarState.activeContainerId;
  const indicatorGap = activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT ? 12 : 4;

  const { preview, closestEdge } = useDraggableActivityBarButton({
    id: props.id,
    order: props.order,
    isDraggable,
    ref: activityButtonRef,
  });

  const handleClick = (id: string) => {
    if (!currentWorkspaceId) return;

    if (isActive && layout?.sidebarState.visible) {
      updateLayout({
        layout: { sidebarState: { visible: false } },
        workspaceId: currentWorkspaceId,
      });
    } else {
      updateLayout({
        layout: {
          activitybarState: { activeContainerId: id },
          sidebarState: { visible: true },
        },
        workspaceId: currentWorkspaceId,
      });
    }
  };

  return (
    <button
      ref={activityButtonRef}
      className={cn("relative flex size-7 cursor-pointer items-center justify-center rounded-md p-1", {
        "hover:background-(--moss-secondary-background-hover)": !isActive || !layout?.sidebarState.visible,
        "background-(--moss-accent-secondary)": isActive && layout?.sidebarState.visible,
      })}
      onClick={() => handleClick(props.id)}
      {...props}
    >
      <div className="size-4.5 flex items-center justify-center [&>svg]:size-full">
        {isActive && layout?.sidebarState.visible && iconActive ? iconActive : icon}
      </div>

      {closestEdge && <DropIndicator edge={closestEdge} gap={indicatorGap} />}

      {preview &&
        createPortal(
          <ActivityBarButton
            {...props}
            icon={icon}
            iconActive={iconActive}
            className="background-(--moss-secondary-background-hover) rounded-md p-1"
            isDraggable={false}
          />,
          preview
        )}
    </button>
  );
};
