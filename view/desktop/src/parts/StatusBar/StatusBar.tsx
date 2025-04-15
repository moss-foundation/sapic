import { useEffect, useRef, useState, type ComponentPropsWithoutRef } from "react";
import { createPortal } from "react-dom";

import { Icon, Icons } from "@/components";
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

import { DropIndicator } from "../../components/DropIndicator";

interface Item {
  id: number;
  icon: Icons;
  label: string;
  order: number;
}

const StatusBar = ({ className }: ComponentPropsWithoutRef<"div">) => {
  const [DNDList, setDNDList] = useState<Item[]>([
    {
      id: 1,
      icon: "StatusBarConsole",
      label: "Console",
      order: 1,
    },
    {
      id: 2,
      icon: "StatusBarTrash",
      label: "Trash",
      order: 2,
    },
    {
      id: 3,
      icon: "StatusBarCookies",
      label: "Cookies",
      order: 3,
    },
  ]);

  useEffect(() => {
    return monitorForElements({
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target || target.data.draggableType !== "StatusBarButton") return;

        const sourceData = source.data;
        const targetData = target.data;
        if (!sourceData || !targetData) return;

        const updatedItems = swapListById(sourceData.id as number, targetData.id as number, DNDList);
        if (!updatedItems) return;

        setDNDList(updatedItems as Item[]);
      },
    });
  }, [DNDList]);

  return (
    <footer
      className={cn(
        "background-(--moss-secondary-background) flex w-screen justify-between border-t border-t-(--moss-border-color) pl-3.5",
        className
      )}
    >
      <div className="flex h-full">
        <div className="flex h-full gap-1">
          {DNDList.map((item) => (
            <StatusBarButton
              key={item.id}
              {...item}
              isDraggable
              draggableType="StatusBarButton"
              className="text-black"
              iconClassName="text-(--moss-icon-primary-text)"
            />
          ))}
        </div>
      </div>

      <div className="flex h-full gap-1">
        <StatusBarButton label="UTF-8" />
        <StatusBarButton label="24 Ln, 16 Col" />
        <StatusBarButton label="4 Spaces" />
        <StatusBarButton label="Rust" />

        <div className="group flex h-full items-center gap-1 px-2 text-white transition hover:bg-white hover:bg-white/10 focus:bg-white focus:bg-white/10">
          <StatusCircle className="size-[6px] bg-[#D62A18]" />
          <span>2 Errors</span>
        </div>

        <div className="group flex h-full items-center gap-1 px-2 text-white transition hover:bg-white hover:bg-white/10 focus:bg-white focus:bg-white/10">
          <StatusCircle className="size-[6px] bg-[#FFC505]" />
          <span>15 Warnings</span>
        </div>

        <StatusBarButton label="--READ--" />
      </div>
    </footer>
  );
};

interface StatusBarButtonProps extends Omit<ComponentPropsWithoutRef<"button">, "id"> {
  icon?: Icons;
  label?: string;
  className?: string;
  iconClassName?: string;

  id?: number;
  isDraggable?: boolean;
  draggableType?: string;
}

const StatusCircle = ({ className }: { className?: string }) => {
  return <div className={cn("flex items-center justify-center rounded-full", className)} />;
};

const StatusBarButton = ({
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
    <button
      ref={ref}
      {...props}
      className={cn("group relative flex h-full items-center justify-center text-white", className)}
    >
      <div className="hover:background-(--moss-icon-primary-background-hover) flex items-center gap-1.5 rounded px-1.5 py-1 transition">
        {icon && <Icon className={cn("my-auto size-[14px] flex-shrink-0", iconClassName)} icon={icon} />}
        {label && <span className="inline-block flex-shrink-0 align-middle leading-[14px]">{label}</span>}
      </div>
      {closestEdge ? <DropIndicator edge={closestEdge} gap={4} /> : null}
      {preview && createPortal(<StatusBarButton icon={icon} label={label} className="bg-sky-500" />, preview)}
    </button>
  );
};

export default StatusBar;
