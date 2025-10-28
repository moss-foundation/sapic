import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { ACTIVITYBAR_POSITION } from "@/constants/layoutPositions";
import { Icon } from "@/lib/ui/Icon";
import { ActivityBarItemProps, useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import DropIndicator from "../DropIndicator";
import { IconInline } from "../IconInline";

export const ActivityBarButton = ({
  icon,
  iconActive,
  isActive,
  isVisible: _,
  isDraggable = true,
  ...props
}: ActivityBarItemProps) => {
  const ref = useRef<HTMLButtonElement | null>(null);

  const { position, setActiveItem } = useActivityBarStore();
  const { setVisible, visible: isSideBarVisible } = useAppResizableLayoutStore((state) => state.sideBar);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

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
                position === ACTIVITYBAR_POSITION.TOP || position === ACTIVITYBAR_POSITION.BOTTOM
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
  }, [position, icon, props, isDraggable]);

  const handleClick = (id: string) => {
    if (isActive && position === ACTIVITYBAR_POSITION.DEFAULT && isSideBarVisible) {
      setVisible(false);
      return;
    }

    setActiveItem(id);
    setVisible(true);
  };

  return (
    <button
      ref={ref}
      className={cn("relative flex size-7 cursor-pointer items-center justify-center rounded-md p-1", {
        "hover:background-(--moss-activityBarItem-background-hover)": !isActive || !isSideBarVisible,
        "background-(--moss-accent-secondary)": isActive && isSideBarVisible,
        "background-(--moss-activityBarItem-background)": !isActive || !isSideBarVisible,
      })}
      onClick={() => handleClick(props.id)}
      {...props}
    >
      {isActive && isSideBarVisible ? (
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
            isActive={false}
            className="background-(--moss-activityBarItem-background-hover) rounded-md p-1"
          />,
          preview
        )}
    </button>
  );
};
