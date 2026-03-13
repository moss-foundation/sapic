import { HTMLAttributes, Ref } from "react";

import { cn } from "@/utils";
import { DragHandleButton } from "@/workbench/ui/components/DragHandleButton";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ActivityIndicator } from "../ActivityIndicator";
import { ReorderDNDIndicator } from "../ReorderDNDIndicator";

interface NodeDetailsProps extends HTMLAttributes<HTMLDivElement> {
  ref?: Ref<HTMLDivElement | null>;
  depth?: number;
  isActive?: boolean;
  reorderInstruction?: Instruction | null;
  hideDragHandle?: boolean;
  isDirty?: boolean;
  nodeOffset?: number;
  paddingRight?: number;
}

export const NodeDetails = ({
  ref,
  depth = 0,
  isActive = false,
  children,
  reorderInstruction = null,
  hideDragHandle = false,
  isDirty = false,
  nodeOffset = 0,
  paddingRight = 0,
  ...props
}: NodeDetailsProps) => {
  const offsetLeft = nodeOffset * depth;

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
      <ActivityIndicator isActive={isActive} isDirty={isDirty} />

      {!hideDragHandle && (
        <DragHandleButton
          className="group-hover/TreeNodeDetails:delay-400 absolute left-[1px] top-1/2 -translate-y-1/2 opacity-0 transition-all duration-0 group-hover/TreeNodeDetails:opacity-100 group-hover/TreeNodeDetails:duration-150"
          slim
          ghost
        />
      )}

      <div style={{ paddingLeft: offsetLeft, paddingRight }} className="flex min-w-0 grow items-center justify-between">
        <ReorderDNDIndicator reorderInstruction={reorderInstruction} offsetLeft={offsetLeft} />
        {children}
      </div>
    </div>
  );
};
