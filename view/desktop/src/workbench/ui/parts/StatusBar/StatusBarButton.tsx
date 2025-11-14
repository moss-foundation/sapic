import { ComponentPropsWithoutRef, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { DropIndicator } from "@/components";
import { Icon, Icons } from "@/lib/ui";
import { cn } from "@/utils";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

interface StatusBarButtonProps extends Omit<ComponentPropsWithoutRef<"button">, "id"> {
  icon?: Icons;
  label?: string;
  className?: string;
  iconClassName?: string;

  id?: number;
  isDraggable?: boolean;
  draggableType?: string;
}

export const StatusBarButton = ({
  icon,
  iconClassName,
  label,
  className,
  id,
  isDraggable,
  draggableType,
  ...props
}: StatusBarButtonProps) => {
  const ref = useRef<HTMLButtonElement | null>(null);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  useEffect(() => {
    const element = ref.current;

    if (!element || !isDraggable) return;

    return combine(
      draggable({
        element: element,
        getInitialData: () => ({ id, icon, label }),
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
        onDrop: () => {
          setClosestEdge(null);
        },
        getData({ input }) {
          return attachClosestEdge(
            { id, label, icon, draggableType },
            {
              element,
              input,
              allowedEdges: ["right", "left"],
            }
          );
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
      })
    );
  }, [id, label, isDraggable, icon, draggableType]);

  return (
    <button ref={ref} {...props} className={cn("relative flex h-full items-center justify-center", className)}>
      <div className="hover:background-(--moss-statusBarItem-background-hover) text-(--moss-statusBarItem-foreground) flex items-center gap-1 rounded py-[3px] pl-1.5 pr-1 transition">
        {icon && <Icon icon={icon} className={cn("size-3.5", iconClassName)} />}
        {label && <span className="">{label}</span>}
      </div>
      {closestEdge ? <DropIndicator edge={closestEdge} gap={4} /> : null}
      {preview && createPortal(<StatusBarButton icon={icon} label={label} className="bg-sky-500" />, preview)}
    </button>
  );
};
