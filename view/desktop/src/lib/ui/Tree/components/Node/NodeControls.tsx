import { forwardRef, HTMLAttributes } from "react";

import { DragHandleButton } from "@/components/DragHandleButton";
import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ActiveNodeIndicator } from "../ActiveNodeIndicator";
import { DropIndicatorForTrigger } from "../DropIndicatorForTrigger";
import { useTreeContext } from "../TreeContext";

interface NodeControlsProps extends HTMLAttributes<HTMLDivElement> {
  depth?: number;
  isChildDropBlocked?: boolean | null;
  isActive?: boolean;
  isRootNode?: boolean;
  instruction?: Instruction | null;
  isLastChild?: boolean;
  hideDragHandle?: boolean;
  dropIndicatorFullWidth?: boolean;
}

export const NodeControls = forwardRef<HTMLDivElement, NodeControlsProps>(
  (
    {
      depth = 0,
      isChildDropBlocked = null,
      isRootNode = false,
      isActive = false,
      children,
      instruction = null,
      isLastChild = false,
      hideDragHandle = false,
      dropIndicatorFullWidth = false,
      ...props
    }: NodeControlsProps,
    ref
  ) => {
    const { nodeOffset, treePaddingLeft, treePaddingRight } = useTreeContext();

    const nodePaddingLeft = depth * nodeOffset + treePaddingLeft;

    return (
      <div
        ref={ref}
        className={cn(
          "group/TreeNodeControls relative flex min-h-[28px] min-w-0 cursor-pointer items-center justify-between"
        )}
        role="button"
        tabIndex={0}
        {...props}
      >
        {isChildDropBlocked !== true && <ActiveNodeIndicator isActive={isActive} />}

        <DropIndicatorForTrigger
          paddingLeft={nodePaddingLeft}
          paddingRight={treePaddingRight}
          instruction={instruction ?? null}
          depth={depth}
          isLastChild={isLastChild}
          fullWidth={dropIndicatorFullWidth}
        />

        {!isRootNode && !hideDragHandle && (
          <DragHandleButton
            className="absolute top-1/2 left-[1px] -translate-y-1/2 opacity-0 transition-all duration-0 group-hover/TreeNodeControls:opacity-100 group-hover/TreeNodeControls:delay-400 group-hover/TreeNodeControls:duration-150"
            slim
            ghost
          />
        )}

        <div
          style={{ paddingLeft: nodePaddingLeft, paddingRight: treePaddingRight }}
          className="flex min-w-0 grow items-center justify-between"
        >
          {children}
        </div>
      </div>
    );
  }
);
