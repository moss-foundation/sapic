import { ComponentPropsWithoutRef, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { DropdownMenu as DM, Icon, Icons } from "@/components";
import { cn } from "@/utils";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { DropIndicator } from "./DropIndicator";

interface ActionsGroupProps extends Omit<ComponentPropsWithoutRef<"div">, "id"> {
  icon?: Icons;
  label?: string;
  compact?: boolean;
  iconClassName?: string;
  defaultAction?: boolean;
  actions?: string[];

  id?: number;
  isDraggable?: boolean;
  draggableType?: string;
}

const buttonStyle =
  "relative hover:border-[var(--moss-widgetBar-dropdown-border)] box-border transition group flex rounded border border-transparent";
const triggerStyle =
  "hover:bg-[var(--moss-widgetBar-item-hover-background)] group flex w-full items-center justify-center gap-1.5 text-ellipsis";
const iconStyle = "group-active:text-[var(--moss-widgetBar-active-text-color)] text-[var(--moss-widgetBar-icon-color)]";
const labelStyle =
  "group-active:text-[var(--moss-widgetBar-active-text-color)] text-[var(--moss-widgetBar-label-color)] break-keep w-max";

export const ActionsGroup = ({
  compact = false,
  defaultAction = false,
  icon,
  label,
  className,
  iconClassName,
  id,
  isDraggable,
  draggableType,
  ...props
}: ActionsGroupProps) => {
  const [open, setOpen] = useState(false);

  const showActions = props.actions !== undefined && props.actions.length > 1;
  const ref = useRef<HTMLDivElement | null>(null);
  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const dropIndicatorGap = 2;

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

  if (!defaultAction) {
    return (
      <div
        ref={ref}
        className={cn(buttonStyle, className, {
          "border-[var(--moss-widgetBar-dropdown-border)]": open,
        })}
        {...props}
      >
        <DM.Root open={open} onOpenChange={() => {}}>
          <DM.Trigger
            className={cn(triggerStyle, "rounded-r px-1.5 py-1", {
              "bg-[var(--moss-widgetBar-item-active-background)]": open,
            })}
            onClick={() => {
              if (showActions) setOpen((prev) => !prev);
            }}
          >
            {icon ? <Icon icon={icon} className={cn(iconStyle, iconClassName)} /> : null}
            {!compact && label && <span className={labelStyle}>{label}</span>}
            {showActions && <Icon icon="ArrowheadDown" className="ml-auto" />}
          </DM.Trigger>

          {showActions && (
            <DM.Content
              className="z-50 flex flex-col border border-[var(--moss-widgetBar-dropdown-border)] bg-[var(--moss-widgetBar-dropdown-background)] text-[var(--moss-widgetBar-dropdown-text)]"
              onPointerDownOutside={() => setOpen(false)}
            >
              {props.actions?.map((id) => (
                <button key={id} className="px-3 py-2 hover:bg-[var(--moss-widgetBar-item-hover-background)]">
                  Action {id}
                </button>
              ))}
            </DM.Content>
          )}
        </DM.Root>
        {closestEdge ? <DropIndicator edge={closestEdge} gap={dropIndicatorGap} /> : null}
        {preview &&
          createPortal(<ActionsGroup icon={icon || undefined} label={label} className="bg-sky-500" />, preview)}
      </div>
    );
  }

  return (
    <div
      ref={ref}
      className={cn(buttonStyle, className, {
        "border-[var(--moss-widgetBar-dropdown-border)]": open,
      })}
      {...props}
    >
      <div className="flex items-stretch">
        <button className={cn(triggerStyle, "px-1.5 py-1")}>
          {icon ? <Icon icon={icon} className={cn(iconStyle, iconClassName)} /> : null}
          {!compact && label && <span className={labelStyle}>{label}</span>}
        </button>

        {showActions && (
          <>
            <div
              className={cn(
                "flex min-w-px grow self-stretch bg-transparent group-hover:bg-[var(--moss-widgetBar-dropdown-border)]",
                {
                  "bg-[var(--moss-widgetBar-dropdown-border)]": open,
                }
              )}
            />
            <DM.Root open={open}>
              <DM.Trigger
                className={cn(triggerStyle, "self-stretch rounded-r", {
                  "bg-[var(--moss-widgetBar-item-active-background)]": open,
                })}
                onClick={() => setOpen((prev) => !prev)}
              >
                <Icon icon="ArrowheadDown" />
              </DM.Trigger>

              <DM.Content
                className="z-50 flex flex-col border border-[var(--moss-widgetBar-dropdown-border)] bg-[var(--moss-widgetBar-dropdown-background)] text-[var(--moss-widgetBar-dropdown-text)]"
                onPointerDownOutside={() => setOpen(false)}
              >
                {props.actions?.map((id) => (
                  <button key={id} className="px-3 py-2 hover:bg-[var(--moss-widgetBar-item-hover-background)]">
                    Action {id}
                  </button>
                ))}
              </DM.Content>
            </DM.Root>
          </>
        )}
      </div>
      {closestEdge ? <DropIndicator edge={closestEdge} gap={dropIndicatorGap} /> : null}
      {preview && createPortal(<ActionsGroup icon={icon || undefined} label={label} />, preview)}
    </div>
  );
};

export default ActionsGroup;
