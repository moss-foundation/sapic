import { ComponentPropsWithoutRef, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { ACTIVITYBAR_POSITION } from "@/constants/layoutPositions";
import { Icon } from "@/lib/ui/Icon";
import { ActivityBarItem, useActivityBarStore } from "@/store/activityBar";
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

export const ActivityBarButton = ({
  icon,
  iconActive,
  isActive,
  visible: _visible, // Extract visible to prevent it from being passed to DOM
  ...props
}: ActivityBarItem & ComponentPropsWithoutRef<"button">) => {
  const ref = useRef<HTMLButtonElement | null>(null);
  const { position, setActiveItem } = useActivityBarStore();
  const { setVisible, visible } = useAppResizableLayoutStore((state) => state.sideBar);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const handleClick = (id: string) => {
    if (isActive && position === ACTIVITYBAR_POSITION.DEFAULT && visible) {
      setVisible(false);
      return;
    }

    setActiveItem(id);
    setVisible(true);
  };

  useEffect(() => {
    const element = ref.current;

    if (!element) return;

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
  }, [position, icon, props]);

  return (
    <button
      ref={ref}
      className={cn(
        "background-(--moss-icon-primary-background) relative flex size-7 cursor-pointer items-center justify-center rounded-md p-1",
        {
          "hover:background-(--moss-icon-primary-background-hover) text-(--moss-icon-primary-text)":
            !isActive || !visible,
          "background-(--moss-icon-primary-background-active) text-(--moss-info-icon)": isActive && visible,
        }
      )}
      onClick={() => handleClick(props.id)}
      {...props}
    >
      <Icon
        icon={isActive && visible ? iconActive : icon}
        className={cn({
          "size-5": position === ACTIVITYBAR_POSITION.DEFAULT,
        })}
      />
      {closestEdge && <DropIndicator edge={closestEdge} gap={12} />}
      {preview &&
        createPortal(
          <ActivityBarButton
            {...props}
            icon={icon}
            iconActive={iconActive}
            isActive={false}
            className="background-(--moss-icon-primary-background-hover) rounded-md p-1"
          />,
          preview
        )}
    </button>
  );
};
