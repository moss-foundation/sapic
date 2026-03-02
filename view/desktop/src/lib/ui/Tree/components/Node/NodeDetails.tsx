import { HTMLAttributes, RefObject } from "react";

import { cn } from "@/utils";
import { DragHandleButton } from "@/workbench/ui/components/DragHandleButton";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropIndicatorForTrigger } from "../DropIndicatorForTrigger";
import { NodeIndicator } from "../NodeIndicator";

interface NodeDetailsProps extends HTMLAttributes<HTMLDivElement> {
  ref?: RefObject<HTMLDivElement | null>;
  depth?: number;
  isChildDropBlocked?: boolean | null;
  isActive?: boolean;
  isRootNode?: boolean;
  instruction?: Instruction | null;
  isLastChild?: boolean;
  hideDragHandle?: boolean;
  dropIndicatorFullWidth?: boolean;
  isDirty?: boolean;
  nodeOffset?: number;
  nodePaddingLeft?: number;
  treePaddingRight?: number;
}

export const NodeDetails = ({
  ref,
  depth = 0,
  isChildDropBlocked = null,
  isRootNode = false,
  isActive = false,
  children,
  instruction = null,
  isLastChild = false,
  hideDragHandle = false,
  dropIndicatorFullWidth = false,
  isDirty = false,
  nodePaddingLeft = 12,
  treePaddingRight = 8,
  ...props
}: NodeDetailsProps) => {
  return (
    <div
      ref={ref}
      className={cn(
        "group/TreeNodeDetails relative flex min-h-[28px] min-w-0 cursor-pointer items-center justify-between"
      )}
      role="button"
      tabIndex={0}
      {...props}
    >
      {isChildDropBlocked !== true && <NodeIndicator isActive={isActive} isDirty={isDirty} />}

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
          className="group-hover/TreeNodeDetails:delay-400 absolute left-[1px] top-1/2 -translate-y-1/2 opacity-0 transition-all duration-0 group-hover/TreeNodeDetails:opacity-100 group-hover/TreeNodeDetails:duration-150"
          slim
          ghost
        />
      )}

      <div
        style={{ paddingLeft: nodePaddingLeft * depth, paddingRight: treePaddingRight }}
        className="flex min-w-0 grow items-center justify-between"
      >
        {children}
      </div>
    </div>
  );
};
