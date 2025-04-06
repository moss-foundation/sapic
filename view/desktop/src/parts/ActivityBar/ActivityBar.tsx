import { ComponentPropsWithoutRef, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { DropIndicator } from "@/components";
import { Icon } from "@/components/Icon";
import { ActivityBarItem, useActivityBarStore } from "@/store/activityBar";
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
  const { items, position, setPosition, setItems } = useActivityBarStore();

  useEffect(() => {
    return monitorForElements({
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target) return;

        const sourceData = source.data;
        const targetData = target.data;
        const edge = extractClosestEdge(targetData);

        if (!sourceData || !targetData) return;

        const updatedItems = swapListById(sourceData.data.id, targetData.data.id, items, edge);

        if (!updatedItems) return;

        setItems(updatedItems);
      },
    });
  }, [items, setItems]);

  return (
    <div
      className={cn(
        "background-(--moss-secondary-background) flex h-full flex-col items-center gap-3 border-r border-r-(--moss-border-color) p-1.5"
        // leftPosition ? "border-r border-r-(--moss-border-color)" : "border-l border-l-(--moss-border-color)",
      )}
    >
      {items.map((item) => (
        <ActivityBarButton key={item.id} {...item} />
      ))}
    </div>
  );
};

const ActivityBarButton = ({ icon, isActive, ...props }: ActivityBarItem & ComponentPropsWithoutRef<"button">) => {
  const { alignment, setItems, items } = useActivityBarStore();
  const ref = useRef<HTMLButtonElement | null>(null);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const handleClick = (id: string) => {
    setItems(
      items.map((item) => {
        return {
          ...item,
          isActive: item.id === id,
        };
      })
    );
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
        "background-(--moss-icon-primary-background-active) text-white": isActive,
        "hover:background-(--moss-icon-primary-background-hover) text-(--moss-icon-primary-text)": !isActive,
      })}
      onClick={() => handleClick(props.id)}
      {...props}
    >
      <Icon icon={icon} className="size-5" />
      {closestEdge ? <DropIndicator edge={closestEdge} gap={12} /> : null}
      {preview && createPortal(<ActivityBarButton {...props} icon={icon} isActive={false} className="" />, preview)}
    </button>
  );
};

export default ActivityBar;
