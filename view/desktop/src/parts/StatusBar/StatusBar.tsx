import { useEffect, useRef, useState, type ComponentPropsWithoutRef } from "react";
import { createPortal } from "react-dom";

import { Icon, Icons } from "@/components";
import { Divider } from "@/components/Divider";
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
import { StatusBarActivity } from "./StatusBarActivity";

interface Item {
  id: number;
  icon: Icons;
  label: string;
  order: number;
}

const StatusBar = ({ className }: ComponentPropsWithoutRef<"div">) => {
  const [isOnline, setIsOnline] = useState(true);
  const [DNDList, setDNDList] = useState<Item[]>([
    {
      id: 1,
      icon: "Console",
      label: "Console",
      order: 1,
    },
    {
      id: 2,
      icon: "Trash",
      label: "Trash",
      order: 2,
    },
    {
      id: 3,
      icon: "Cookie",
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

  useEffect(() => {
    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);

    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  return (
    <footer
      className={cn(
        "background-(--moss-secondary-background) flex w-screen justify-between border-t border-t-(--moss-border-color) pr-4 pl-3.5",
        className
      )}
    >
      <div className="flex h-full">
        <div className="flex h-full gap-1.5">
          {DNDList.map((item) => (
            <StatusBarButton
              key={item.id}
              {...item}
              isDraggable
              draggableType="StatusBarButton"
              className="text-(--moss-statusBar-icon-primary-text)"
              iconClassName="text-(--moss-statusBar-icon-secondary-text)"
            />
          ))}
        </div>
        <Divider height="medium" />

        <StatusBarIndicators />
        <StatusBarActivity />
      </div>

      <div className="flex h-full gap-0.5">
        <StatusBarButton label="60 FPS" className="text-(--moss-statusBar-icon-primary-text)" />
        <Divider height="medium" />
        <StatusBarButton
          icon={isOnline ? "Success" : "Error"}
          label={isOnline ? "Online" : "Offline"}
          className="text-(--moss-statusBar-icon-primary-text)"
          iconClassName={isOnline ? "text-[#1E6B33]" : "text-[#DF9303]"}
        />
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

const StatusBarIndicators = () => {
  return (
    <div className="flex h-full items-center">
      <button className="group flex h-full items-center">
        <div className="flex items-center rounded px-1 transition">
          <div className="hover:background-(--moss-statusBar-icon-background-hover) flex h-[22px] items-center space-x-2 rounded px-1">
            <div className="flex items-center gap-1">
              <Icon className="size-[14px] text-[#E55765]" icon="Failed" />
              <span className="text-sm text-(--moss-statusBar-icon-primary-text)">2</span>
            </div>
            <div className="flex items-center gap-1">
              <Icon className="size-[14px] text-[#FFAF0F]" icon="Warning" />
              <span className="text-sm text-(--moss-statusBar-icon-primary-text)">5</span>
            </div>
          </div>
        </div>
      </button>
    </div>
  );
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
      <div className="hover:background-(--moss-statusBar-icon-background-hover) flex h-[22px] items-center gap-1 rounded px-1.5 transition">
        {icon && <Icon className={cn("my-auto size-[14px] flex-shrink-0", iconClassName)} icon={icon} />}
        {label && <span className="inline-block flex-shrink-0 align-middle leading-[14px]">{label}</span>}
      </div>
      {closestEdge ? <DropIndicator edge={closestEdge} gap={4} /> : null}
      {preview && createPortal(<StatusBarButton icon={icon} label={label} className="bg-sky-500" />, preview)}
    </button>
  );
};

export default StatusBar;
