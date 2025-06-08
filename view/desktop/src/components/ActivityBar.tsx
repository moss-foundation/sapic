import { ComponentPropsWithoutRef, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { Icon } from "@/lib/ui/Icon";
import { ActivityBarItem, useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn, swapListById } from "@/utils";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import {
  draggable,
  dropTargetForElements,
  monitorForElements,
} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import DropIndicator from "./DropIndicator";

export const ActivityBar = () => {
  const { items, position, setItems } = useActivityBarStore();
  const sideBarPosition = useAppResizableLayoutStore((state) => state.sideBarPosition);
  const { visible } = useAppResizableLayoutStore((state) => state.sideBar);

  useEffect(() => {
    return monitorForElements({
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target) return;

        const sourceData = source.data as { data: ActivityBarItem };
        const targetData = target.data as { data: ActivityBarItem };
        const edge = extractClosestEdge(targetData);

        if (!sourceData || !targetData || !sourceData.data || !targetData.data) return;

        const updatedItems = swapListById(sourceData.data.id, targetData.data.id, items, edge);

        if (!updatedItems) return;

        setItems(updatedItems);
      },
    });
  }, [items, setItems]);

  return (
    <div
      className={cn("background-(--moss-secondary-background) flex items-center gap-3", {
        "w-full border-b border-b-(--moss-border-color) px-1.5": position === ACTIVITYBAR_POSITION.TOP,
        "w-full border-t border-t-(--moss-border-color) px-1.5": position === ACTIVITYBAR_POSITION.BOTTOM,
        "h-full flex-col py-1.5": position === ACTIVITYBAR_POSITION.DEFAULT,
        "hidden": position === ACTIVITYBAR_POSITION.HIDDEN,

        "border-l border-l-(--moss-border-color)":
          sideBarPosition === SIDEBAR_POSITION.RIGHT && position === ACTIVITYBAR_POSITION.DEFAULT,
      })}
    >
      {items
        .filter((item) => item.visible !== false) // Only show visible items
        .map((item) => (
          <div
            key={item.id}
            className={cn("relative flex flex-col", {
              "px-1.5": position === ACTIVITYBAR_POSITION.DEFAULT,
              "py-1.5": position === ACTIVITYBAR_POSITION.TOP || position === ACTIVITYBAR_POSITION.BOTTOM,
            })}
          >
            <ActivityBarButton key={item.id} {...item} />

            {item.isActive && visible && <ActivityBarButtonIndicator />}
          </div>
        ))}
    </div>
  );
};

const ActivityBarButton = ({
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
      className={cn("relative cursor-pointer rounded-md p-1", {
        "background-(--moss-icon-primary-background-active) text-(--moss-info-icon)": isActive && visible,
        "hover:background-(--moss-icon-primary-background-hover) text-(--moss-icon-primary-text)":
          !isActive || !visible,
      })}
      onClick={() => handleClick(props.id)}
      {...props}
    >
      <Icon icon={isActive && visible ? iconActive : icon} className="size-5" />
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

const ActivityBarButtonIndicator = () => {
  const position = useActivityBarStore((state) => state.position);
  const sideBarPosition = useAppResizableLayoutStore((state) => state.sideBarPosition);

  return (
    <div
      className={cn("absolute shadow-[inset_0_-2px_10px_var(--moss-primary)] transition-[height,width] duration-300", {
        "bottom-0 left-1/2 h-0.5 w-2.5 -translate-x-1/2 rounded-t-[10px] [button:hover_+_&]:w-full":
          position === ACTIVITYBAR_POSITION.TOP,
        "top-0 left-1/2 h-0.5 w-2.5 -translate-x-1/2 rounded-b-[10px] [button:hover_+_&]:w-full":
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

export default ActivityBar;
