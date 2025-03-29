import { ComponentPropsWithoutRef, forwardRef, useCallback, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { ActivityBarState } from "@/hooks/useActivityBarState";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useChangeActivityBarState } from "@/hooks/useChangeActivityBarState";
import { useChangeProjectSessionState, useGetProjectSessionState } from "@/hooks/useProjectSession";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useGetViewGroups } from "@/hooks/useGetViewGroups";
import { cn } from "@/utils";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { attachClosestEdge, extractClosestEdge, Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import Icon, { Icons } from "./Icon";

const positions = ["default", "top", "bottom", "hidden"] as const;

interface ViewGroup {
  id: string;
  icon: string;
}

type EdgeDirection = "top" | "bottom" | "left" | "right" | null;

export const ActivityBar = () => {
  const { data: activityBarState } = useGetActivityBarState();
  const { mutate: changeActivityBarState } = useChangeActivityBarState();
  const { data: appLayoutState } = useGetAppLayoutState();

  const { data: viewGroups } = useGetViewGroups();
  const { data: projectSessionState } = useGetProjectSessionState();
  const { mutate: changeProjectSessionState } = useChangeProjectSessionState();

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

    if (appLayoutState.activeSidebar === "none") {
      return "left";
    }

    return appLayoutState.activeSidebar === "right" ? ("right" as const) : ("left" as const);
  };

  const effectivePosition = getEffectivePosition();

  const toggleActiveGroup = useCallback(
    (id: string) => {
      if (!projectSessionState) return;

      changeProjectSessionState({
        ...projectSessionState,
        lastActiveGroup: id,
      });
    },
    [changeProjectSessionState, projectSessionState]
  );

  const handleSelectPosition = (position: ActivityBarState["position"]) => {
    const currentIndex = positions.indexOf(position);

    const activeGroup = projectSessionState?.lastActiveGroup;

    if (currentIndex === -1 || currentIndex >= positions.length - 1) {
      changeActivityBarState({
        position: positions[0],
        groupOrder: activityBarState?.groupOrder || [],
      });
    } else {
      changeActivityBarState({
        position: positions[currentIndex + 1],
        groupOrder: activityBarState?.groupOrder || [],
      });
    }

    if (activeGroup && projectSessionState && changeProjectSessionState) {
      // Small delay to ensure the state update happens after position change
      setTimeout(() => {
        changeProjectSessionState({
          ...projectSessionState,
          lastActiveGroup: activeGroup,
        });
      }, 0);
    }
  };

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
          "flex h-[35px] w-full items-center gap-2 bg-[var(--moss-activityBar-background)] px-2 py-1",
          topPosition
            ? "border-b border-b-[var(--moss-activityBar-border-color)]"
            : "border-t border-t-[var(--moss-activityBar-border-color)]",
          effectivePosition === "bottom" && "absolute right-0 bottom-0 left-0 z-10"
        )}
        onDoubleClick={() => handleSelectPosition(activityBarState?.position || "default")}
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
        "flex h-full w-[41px] flex-col items-center gap-2 bg-[var(--moss-activityBar-background)] px-1 py-2",
        leftPosition
          ? "border-r border-r-[var(--moss-activityBar-border-color)]"
          : "border-l border-l-[var(--moss-activityBar-border-color)]",
        "flex-shrink-0" // Prevent shrinking when sidebar is resized
      )}
      onDoubleClick={() => handleSelectPosition(activityBarState?.position || "default")}
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
          "bg-[var(--moss-activityBar-active-item)]": active,
          "hover:bg-[var(--moss-activityBar-hover-item)]": !active && !isDragging,
          "opacity-50": isBeingDragged,
          "cursor-grabbing": isDragActive,
        })}
        data-draggable-id={id}
        data-draggable-type={draggableType}
        data-draggable-index={index}
        data-orientation={orientation}
      >
        <Icon
          icon={icon}
          className={cn(
            {
              "text-[var(--moss-activityBar-active-icon)]": active,
              "text-[var(--moss-activityBar-icon-inactive)]": !active,
            },
            iconClassName
          )}
        />
        {isDraggable && closestEdge && !isBeingDragged && (
          <div
            className={cn("absolute z-10 bg-[var(--moss-activityBar-indicator-color)]", {
              "-top-1 right-0 left-0 h-0.5": closestEdge === "top",
              "right-0 -bottom-1 left-0 h-0.5": closestEdge === "bottom",
              "top-0 bottom-0 -left-1 w-0.5": closestEdge === "left",
              "top-0 -right-1 bottom-0 w-0.5": closestEdge === "right",
            })}
          />
        )}
        {preview &&
          createPortal(
            <div className="flex size-7 items-center justify-center rounded-md bg-[var(--moss-activityBar-active-item)] shadow-lg">
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
