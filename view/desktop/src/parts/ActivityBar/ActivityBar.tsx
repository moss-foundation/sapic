import { ComponentPropsWithoutRef, forwardRef, useCallback, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { Icon, Icons } from "@/components/Icon";
import { ActivityBarState } from "@/hooks/useActivityBarState";
import { useChangeActivityBarState } from "@/hooks/useChangeActivityBarState";
import { useChangeAppLayoutState } from "@/hooks/useChangeAppLayoutState";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useGetViewGroups } from "@/hooks/useGetViewGroups";
import { useChangeProjectSessionState, useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";
import { attachClosestEdge, Edge, extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import {
  draggable,
  dropTargetForElements,
  monitorForElements,
} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

interface ViewGroup {
  id: string;
  icon: string;
}

type EdgeDirection = "top" | "bottom" | "left" | "right" | null;

export const ActivityBar = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { mutate: changeActivityBarState } = useChangeActivityBarState();
  const { data: appLayoutState } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();
  const sideBarSetWidth = useAppResizableLayoutStore((state) => state.sideBar.setWidth);
  const sideBarGetWidth = useAppResizableLayoutStore((state) => state.sideBar.getWidth);

  const { data: viewGroups } = useGetViewGroups();
  const { data: projectSessionState } = useGetProjectSessionState();
  const { mutate: changeProjectSessionState } = useChangeProjectSessionState();

  const lastSidebarWidthRef = useRef(sideBarGetWidth() || 270);

  useEffect(() => {
    if (appLayoutState?.activeSidebar !== "none" && sideBarGetWidth() > 45) {
      lastSidebarWidthRef.current = sideBarGetWidth();
    }
  }, [appLayoutState?.activeSidebar, sideBarGetWidth]);

  const activityBarGroups = viewGroups?.viewGroups as ViewGroup[] | undefined;
  const [orderedGroups, setOrderedGroups] = useState<string[]>([]);
  const DNDListRef = useRef<HTMLDivElement>(null);
  const orderedGroupsInitialized = useRef(false);
  const [draggedItemId, setDraggedItemId] = useState<string | null>(null);

  useEffect(() => {
    if (!activityBarGroups || activityBarGroups.length === 0 || !activityBarState) return;

    if (orderedGroupsInitialized.current) return;

    try {
      const savedOrder = activityBarState.groupOrder;

      if (savedOrder && savedOrder.length > 0) {
        const validOrder = savedOrder.filter((id: string) => activityBarGroups.some((group) => group.id === id));

        const newGroupIds = activityBarGroups.map((group) => group.id).filter((id: string) => !validOrder.includes(id));

        const finalOrder = [...validOrder, ...newGroupIds];

        if (finalOrder.length === activityBarGroups.length) {
          setOrderedGroups(finalOrder);
          orderedGroupsInitialized.current = true;

          if (!projectSessionState?.lastActiveGroup) {
            const firstGroup = finalOrder[0];
            if (firstGroup && changeProjectSessionState && projectSessionState) {
              changeProjectSessionState({
                ...projectSessionState,
                lastActiveGroup: firstGroup,
              });
            }
          }
          return;
        }
      }
    } catch (error) {
      console.error("Error loading ActivityBar order", error);
    }

    const initialOrder = activityBarGroups.map((group) => group.id);
    setOrderedGroups(initialOrder);
    orderedGroupsInitialized.current = true;

    if (!projectSessionState?.lastActiveGroup) {
      const firstGroup = initialOrder[0];
      if (firstGroup && changeProjectSessionState && projectSessionState) {
        changeProjectSessionState({
          ...projectSessionState,
          lastActiveGroup: firstGroup,
        });
      }
    }
  }, [activityBarGroups, projectSessionState, changeProjectSessionState, activityBarState]);

  useEffect(() => {
    if (orderedGroups.length === 0) return;

    try {
      changeActivityBarState({ groupOrder: orderedGroups });
    } catch (error) {
      console.error("Error saving ActivityBar order", error);
    }
  }, [orderedGroups, changeActivityBarState]);

  useEffect(() => {
    if (!orderedGroups.length) return;

    const cleanup = monitorForElements({
      onDragStart({ source }) {
        setDraggedItemId((source.data.id as string) || null);
      },
      onDrop({ location, source }) {
        setDraggedItemId(null);

        if (!location.current.dropTargets.length) {
          return;
        }

        const target = location.current.dropTargets[0];
        const sourceId = source.data.id as string;
        const targetId = target.data.id as string;

        if (!sourceId || !targetId || sourceId === targetId) {
          return;
        }

        const sourceIndex = orderedGroups.indexOf(sourceId);
        const targetIndex = orderedGroups.indexOf(targetId);

        if (sourceIndex === -1 || targetIndex === -1) {
          return;
        }

        const newOrderedGroups = [...orderedGroups];

        const [removed] = newOrderedGroups.splice(sourceIndex, 1);

        const closestEdge = extractClosestEdge(target.data);

        const adjustedTargetIndex = sourceIndex < targetIndex ? targetIndex - 1 : targetIndex;

        if (closestEdge === "left" || closestEdge === "top") {
          newOrderedGroups.splice(adjustedTargetIndex, 0, removed);
        } else {
          newOrderedGroups.splice(adjustedTargetIndex + 1, 0, removed);
        }

        setOrderedGroups(newOrderedGroups);

        // Make the dragged item active after dropping
        if (projectSessionState && changeProjectSessionState) {
          changeProjectSessionState({
            ...projectSessionState,
            lastActiveGroup: sourceId,
          });
        }
      },
    });

    return cleanup;
  }, [orderedGroups, projectSessionState, changeProjectSessionState]);

  const getEffectivePosition = () => {
    if (!activityBarState || !appLayoutState) return "left";

    if (activityBarState.position !== "default") {
      return activityBarState.position;
    }

    // When in default mode, determine position based on sidebarSetting
    // This ensures the ActivityBar shows in the correct position
    // even when sidebar is hidden (activeSidebar === "none")
    return appLayoutState.sidebarSetting === "right" ? "right" : "left";
  };

  const effectivePosition = getEffectivePosition();

  const toggleActiveGroup = useCallback(
    (id: string) => {
      if (!projectSessionState) return;

      const isAlreadyActive = projectSessionState.lastActiveGroup === id;

      const isVerticalDefault =
        activityBarState?.position === "default" && (effectivePosition === "left" || effectivePosition === "right");

      if (isAlreadyActive && isVerticalDefault) {
        if (appLayoutState?.activeSidebar !== "none") {
          changeAppLayoutState({
            activeSidebar: "none",
            sidebarSetting: appLayoutState?.sidebarSetting || "left",
          });
        } else {
          changeAppLayoutState({
            activeSidebar: appLayoutState?.sidebarSetting || "left",
            sidebarSetting: appLayoutState?.sidebarSetting || "left",
          });
          sideBarSetWidth(lastSidebarWidthRef.current);
        }
        return;
      }

      if (isAlreadyActive) {
        return;
      }

      changeProjectSessionState({
        ...projectSessionState,
        lastActiveGroup: id,
      });

      if (appLayoutState?.activeSidebar === "none" && appLayoutState?.sidebarSetting) {
        changeAppLayoutState({
          activeSidebar: appLayoutState.sidebarSetting,
          sidebarSetting: appLayoutState.sidebarSetting,
        });

        sideBarSetWidth(lastSidebarWidthRef.current);
      }
    },
    [
      changeProjectSessionState,
      projectSessionState,
      appLayoutState,
      changeAppLayoutState,
      sideBarSetWidth,
      activityBarState,
      effectivePosition,
    ]
  );

  const getOrderedActivityBarGroups = () => {
    if (!activityBarGroups || orderedGroups.length === 0) return activityBarGroups || [];

    return orderedGroups
      .map((id) => activityBarGroups.find((group) => group.id === id))
      .filter((group): group is ViewGroup => group !== undefined);
  };

  const orderedActivityBarGroups = getOrderedActivityBarGroups();

  if (effectivePosition === "hidden") {
    return null;
  }

  // Horizontal orientation (top or bottom)
  if (effectivePosition === "top" || effectivePosition === "bottom") {
    const topPosition = effectivePosition === "top";

    return (
      <div
        ref={DNDListRef}
        className={cn(
          "background-(--moss-secondary-bg) flex h-[35px] w-full items-center gap-2 px-2 py-1",
          topPosition ? "border-b border-b-(--moss-border-color)" : "border-t border-t-(--moss-border-color)",
          effectivePosition === "bottom" && "absolute right-0 bottom-0 left-0 z-10"
        )}
      >
        {orderedActivityBarGroups.map(({ icon, id }, index) => (
          <ActivityBarButton
            key={id}
            id={id}
            icon={icon as Icons}
            active={projectSessionState?.lastActiveGroup === id}
            onClick={() => toggleActiveGroup(id)}
            isDraggable
            draggableType="ActivityBarButton"
            index={index}
            orientation="horizontal"
            isDragActive={draggedItemId !== null}
            isBeingDragged={id === draggedItemId}
            activityBarState={activityBarState}
          />
        ))}
      </div>
    );
  }

  // Vertical orientation (left or right)
  const leftPosition = effectivePosition === "left";

  return (
    <div
      ref={DNDListRef}
      className={cn(
        "background-(--moss-secondary-bg) flex h-full w-[41px] flex-col items-center gap-2 px-1 py-2",
        leftPosition ? "border-r border-r-(--moss-border-color)" : "border-l border-l-(--moss-border-color)",
        "flex-shrink-0"
      )}
    >
      {orderedActivityBarGroups.map(({ icon, id }, index) => (
        <ActivityBarButton
          key={id}
          id={id}
          icon={icon as Icons}
          active={projectSessionState?.lastActiveGroup === id}
          onClick={() => toggleActiveGroup(id)}
          isDraggable
          draggableType="ActivityBarButton"
          index={index}
          orientation="vertical"
          isDragActive={draggedItemId !== null}
          isBeingDragged={id === draggedItemId}
          activityBarState={activityBarState}
        />
      ))}
    </div>
  );
};

interface ActivityBarButtonProps extends ComponentPropsWithoutRef<"div"> {
  icon: Icons;
  active?: boolean;
  iconClassName?: string;
  id: string;
  isDraggable?: boolean;
  draggableType?: string;
  index?: number;
  orientation?: "horizontal" | "vertical";
  isDragActive?: boolean;
  isBeingDragged?: boolean;
  activityBarState?: ActivityBarState;
}

const ActivityBarButton = forwardRef<HTMLDivElement, ActivityBarButtonProps>(
  (
    {
      icon,
      active = false,
      iconClassName,
      id,
      isDraggable,
      draggableType,
      index,
      orientation = "vertical",
      isDragActive = false,
      isBeingDragged = false,
      activityBarState,
      ...props
    },
    ref
  ) => {
    const buttonRef = useRef<HTMLDivElement>(null);
    const elementRef = ref || buttonRef;
    const [preview, setPreview] = useState<HTMLElement | null>(null);
    const [closestEdge, setClosestEdge] = useState<EdgeDirection>(null);
    const [isDragging, setIsDragging] = useState(false);
    const [currentSourceId, setCurrentSourceId] = useState<string | null>(null);

    useEffect(() => {
      const element = "current" in elementRef ? elementRef.current : null;

      if (!element || !isDraggable) return;

      return combine(
        draggable({
          element: element,
          getInitialData: () => ({
            id,
            icon,
            draggableType,
            index,
            orientation,
          }),
          onDragStart() {
            setIsDragging(true);
            setCurrentSourceId(id);
          },
          onDrop: () => {
            setPreview(null);
            setClosestEdge(null);
            setIsDragging(false);
            setCurrentSourceId(null);
          },
          onGenerateDragPreview({ nativeSetDragImage }) {
            setCustomNativeDragPreview({
              nativeSetDragImage,
              render({ container }) {
                setPreview(container);
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
            const allowedEdges: Edge[] = orientation === "horizontal" ? ["left", "right"] : ["top", "bottom"];

            return attachClosestEdge(
              {
                id,
                icon,
                draggableType,
                index,
                orientation,
              },
              {
                element,
                input,
                allowedEdges,
              }
            );
          },
          getIsSticky() {
            return true;
          },
          onDragEnter({ self, source }) {
            // Don't show edge indicators when dragging over self
            if (source?.data?.id === id || currentSourceId === id) return;

            const edge = extractClosestEdge(self.data);
            setClosestEdge(edge);
          },
          onDrag({ self, source }) {
            // Don't show edge indicators when dragging over self
            if (source?.data?.id === id || currentSourceId === id) return;

            const edge = extractClosestEdge(self.data);
            setClosestEdge((current) => {
              if (current === edge) return current;
              return edge;
            });
          },
          onDragLeave() {
            setClosestEdge(null);
          },
        })
      );
    }, [id, isDraggable, icon, draggableType, index, orientation, elementRef, currentSourceId]);

    return (
      <div
        ref={elementRef as React.RefObject<HTMLDivElement>}
        {...props}
        className={cn("relative flex size-7 items-center justify-center rounded-md", {
          "background-(--moss-icon-primary-bg-active) text-white": active,
          "hover:background-(--moss-icon-primary-bg-hover)": !active && !isDragging,
          "opacity-50": isBeingDragged,
          "cursor-grabbing": isDragActive,
        })}
        data-draggable-id={id}
        data-draggable-type={draggableType}
        data-draggable-index={index}
        data-orientation={orientation}
        title={
          active && orientation === "vertical" && activityBarState?.position === "default"
            ? "Click to toggle sidebar visibility"
            : "Click to activate this view"
        }
      >
        <Icon icon={icon} className={cn("size-5", iconClassName)} />
        {isDraggable && closestEdge && !isBeingDragged && (
          <div
            className={cn("background-(--moss-primary) absolute z-10", {
              "-top-1 right-0 left-0 h-0.5": closestEdge === "top",
              "right-0 -bottom-1 left-0 h-0.5": closestEdge === "bottom",
              "top-0 bottom-0 -left-1 w-0.5": closestEdge === "left",
              "top-0 -right-1 bottom-0 w-0.5": closestEdge === "right",
            })}
          />
        )}
        {preview &&
          createPortal(
            <div className="background-(--moss-icon-primary-bg-active) flex size-7 items-center justify-center rounded-md">
              <Icon icon={icon} className="text-[var(--moss-activityBar-active-icon)]" />
            </div>,
            preview
          )}
      </div>
    );
  }
);

ActivityBarButton.displayName = "ActivityBarButton";

export type ActivityBarPosition = "default" | "top" | "bottom" | "hidden";
export default ActivityBar;
