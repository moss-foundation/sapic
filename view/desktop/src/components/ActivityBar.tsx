import { ComponentPropsWithoutRef, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { DropIndicator } from "@/components";
import { Icon } from "@/components/Icon";
import { ActivityBarItem, useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn, swapListById } from "@/utils";
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
        "w-full border-b border-b-(--moss-border-color) px-1.5": position === "top",
        "w-full border-t border-t-(--moss-border-color) px-1.5": position === "bottom",
        "h-full flex-col py-1.5": position === "default",
        "hidden": position === "hidden",

        "border-l border-l-(--moss-border-color)": sideBarPosition === "right" && position === "default",
      })}
    >
      {items.map((item) => (
        <div
          className={cn("relative flex flex-col", {
            "px-1.5": position === "default",
            "py-1.5": position === "top" || position === "bottom",
          })}
        >
          <ActivityBarButton key={item.id} {...item} />

          {item.isActive && visible && <ActivityBarButtonIndicator />}
        </div>
      ))}
    </div>
  );
};

const ActivityBarButton = ({ icon, isActive, ...props }: ActivityBarItem & ComponentPropsWithoutRef<"button">) => {
  const ref = useRef<HTMLButtonElement | null>(null);

  const { alignment, setItems, items, position } = useActivityBarStore();
  const { setVisible, visible } = useAppResizableLayoutStore((state) => state.sideBar);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const handleClick = (id: string) => {
    if (isActive && position === "default" && visible) {
      setVisible(false);
      return;
    }

    setItems(
      items.map((item) => {
        return {
          ...item,
          isActive: item.id === id,
        };
      })
    );
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
              allowedEdges: alignment === "horizontal" ? ["right", "left"] : ["top", "bottom"],
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
  }, [alignment, icon, props]);

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
      <Icon icon={icon} className="size-5" />
      {closestEdge ? <DropIndicator edge={closestEdge} gap={12} /> : null}
      {preview &&
        createPortal(
          <ActivityBarButton
            {...props}
            icon={icon}
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
      className={cn("absolute shadow-[inset_0_-2px_10px_var(--moss-primary)]", {
        "inset-x-[9px] bottom-0 h-0.5 w-2.5 rounded-t-[10px]": position === "top",
        "inset-x-[9px] top-0 h-0.5 w-2.5 rounded-b-[10px]": position === "bottom",
        "inset-y-[9px] h-2.5 w-0.5": position === "default",
        "right-0 rounded-l-[10px]": sideBarPosition === "right" && position === "default",
        "left-0 rounded-r-[10px]": sideBarPosition === "left" && position === "default",
      })}
    />
  );
};

export default ActivityBar;
