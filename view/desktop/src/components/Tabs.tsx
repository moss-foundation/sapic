import { HTMLAttributes, ReactElement, ReactNode, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { cn } from "../utils";
import DropIndicator from "./DropIndicator";
import Scrollbar from "./Scrollbar";

const Tabs = ({ children, className, ...props }: HTMLAttributes<HTMLDivElement>) => {
  return (
    <div className={cn("h-full w-full", className)} {...props}>
      {children}
    </div>
  );
};

interface TabsListProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactElement<React.ComponentProps<typeof Tab>> | ReactElement<React.ComponentProps<typeof Tab>>[];
}

const TabsList = ({ children, className, ...props }: TabsListProps) => {
  return (
    <Scrollbar>
      <div
        role="tablist"
        aria-labelledby="tablist-1"
        data-tabs="default"
        className={cn(`background-[var(--moss-tabslist-background)] relative flex h-full w-full`, className)}
        {...props}
      >
        {children}
      </div>
    </Scrollbar>
  );
};

interface TabProps extends Omit<HTMLAttributes<HTMLButtonElement>, "id"> {
  id: number | string;
  isActive: boolean;
  isDraggable?: boolean;
  label: string;
  draggableType?: string;
}

const Tab = ({
  id,
  isActive,
  isDraggable = false,
  className,
  draggableType = "TabTrigger",
  label,
  ...props
}: TabProps) => {
  const ref = useRef<HTMLButtonElement | null>(null);
  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  useEffect(() => {
    const element = ref?.current;

    if (!element || !isDraggable) return;

    return combine(
      draggable({
        element,
        getInitialData: () => ({ id, label, type: "Tab" }),
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
            { id, label, draggableType },
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
  }, [id, label, isDraggable, draggableType, ref]);

  return (
    <button
      ref={ref}
      id={`${id}`}
      type="button"
      role="tab"
      aria-selected={isActive}
      aria-controls={`panel-${id}`}
      tabIndex={isActive ? 0 : -1}
      className={cn(
        "background-(--moss-tab-background) aria-selected:background-(--moss-active-tab-background) relative box-border min-w-max grow cursor-pointer border-t px-3 pt-[7px] pb-2 text-[var(--moss-tab-text)] select-none",
        {
          "background-(--moss-active-tab-background) border-t-(--moss-primary) text-(--moss-active-tab-text)": isActive,
          "hover:background-(--moss-hover-inactive-tab-background) border-t-transparent": !isActive,
        },
        className
      )}
      {...props}
    >
      <span className="focus">{label}</span>
      {closestEdge ? <DropIndicator edge={closestEdge} gap={0} noTerminal /> : null}
      {preview && createPortal(<Tab id={id} label={label} isActive={isActive} />, preview)}
    </button>
  );
};

interface TabsPanelsProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
}

const TabsPanels = ({ children, className, ...props }: TabsPanelsProps) => {
  return (
    <div
      className={cn(
        "background-(--moss-tab-panels-background)] flex h-full w-full grow flex-col overflow-hidden",
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
};

interface TabPanelProps extends Omit<HTMLAttributes<HTMLDivElement>, "id"> {
  children: ReactNode;
  id: string | number;
  isActive: boolean;
}

const TabPanel = ({ children, id, isActive, className, ...props }: TabPanelProps) => {
  return (
    <div
      id={`panel-${id}`}
      role="tabpanel"
      tabIndex={0}
      aria-labelledby={`${id}`}
      className={cn(
        "flex grow flex-col overflow-auto",
        {
          "hidden": !isActive,
        },
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
};

Tabs.List = TabsList;
Tabs.Tab = Tab;
Tabs.Panels = TabsPanels;
Tabs.Panel = TabPanel;

export default Tabs;
