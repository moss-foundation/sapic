import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { useGetLayout, useUpdateLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION } from "@/workbench/domains/layout";
import { ActivityBarItemProps } from "@/workbench/store/activityBar";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import DropIndicator from "@/workbench/ui/components/DropIndicator";
import { IconInline } from "@/workbench/ui/components/IconInline";

export const ActivityBarButton = ({
  icon,
  iconActive,
  isVisible: _,
  isDraggable = true,
  ...props
}: ActivityBarItemProps) => {
  const ref = useRef<HTMLButtonElement | null>(null);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;
  const isActive = props.id === layout?.activitybarState.activeContainerId;

  useEffect(() => {
    const element = ref.current;

    if (!element || !isDraggable) return;

    return combine(
      draggable({
        element: element,
        getInitialData: () => ({ type: "ActivityBarButton", data: { icon, ...props } }),
        onDrop: () => {
          setPreview(null);
        },
        onGenerateDragPreview({ nativeSetDragImage }) {
          setCustomNativeDragPreview({
            nativeSetDragImage,
            render({ container }) {
              setPreview((prev) => (prev === container ? prev : container));
            },
          });
        },
      }),
      dropTargetForElements({
        element,
        getData({ input }) {
          return attachClosestEdge(
            { type: "ActivityBarButton", data: { icon, ...props } },
            {
              element,
              input,
              allowedEdges:
                activityBarPosition === ACTIVITYBAR_POSITION.TOP || activityBarPosition === ACTIVITYBAR_POSITION.BOTTOM
                  ? ["right", "left"]
                  : ["top", "bottom"],
            }
          );
        },
        canDrop({ source }) {
          return source?.data?.type === "ActivityBarButton";
        },
        getIsSticky() {
          return true;
        },
        onDragEnter({ self }) {
          const closestEdge = extractClosestEdge(self.data);
          setClosestEdge(closestEdge);
        },
        onDrag({ self }) {
          const closestEdge = extractClosestEdge(self.data);

          setClosestEdge((current) => {
            if (current === closestEdge) return current;

            return closestEdge;
          });
        },
        onDragLeave() {
          setClosestEdge(null);
        },
        onDrop: () => {
          setClosestEdge(null);
        },
      })
    );
  }, [activityBarPosition, icon, props, isDraggable]);

  const handleClick = (id: string) => {
    if (!currentWorkspaceId) return;

    if (isActive && layout?.sidebarState.visible) {
      updateLayout({
        layout: {
          sidebarState: {
            visible: false,
          },
        },
        workspaceId: currentWorkspaceId,
      });
    } else {
      updateLayout({
        layout: {
          activitybarState: {
            activeContainerId: id,
          },
          sidebarState: {
            visible: true,
          },
        },
        workspaceId: currentWorkspaceId,
      });
    }
  };

  return (
    <button
      ref={ref}
      className={cn("relative flex size-7 cursor-pointer items-center justify-center rounded-md p-1", {
        "hover:background-(--moss-activityBarItem-background-hover)": !isActive || !layout?.sidebarState.visible,
        "background-(--moss-accent-secondary)": isActive && layout?.sidebarState.visible,
        "background-(--moss-activityBarItem-background)": !isActive || !layout?.sidebarState.visible,
      })}
      onClick={() => handleClick(props.id)}
      {...props}
    >
      {isActive && layout?.sidebarState.visible ? (
        <IconInline icon={iconActive} className="size-4.5" />
      ) : (
        <Icon icon={icon} className="size-4.5" />
      )}

      {closestEdge && <DropIndicator edge={closestEdge} gap={12} />}

      {preview &&
        createPortal(
          <ActivityBarButton
            {...props}
            icon={icon}
            iconActive={iconActive}
            className="background-(--moss-activityBarItem-background-hover) rounded-md p-1"
          />,
          preview
        )}
    </button>
  );
};
