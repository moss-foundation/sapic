import { useRef } from "react";

import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { useDraggableGroupedEnvironmentsList } from "../hooks/useDraggableGroupedEnvironmentsList";
import { GroupedEnvironments } from "../types";
import { GroupedEnvironmentsListChildren } from "./GroupedEnvironmentsListChildren";
import { GroupedEnvironmentsListRootControls } from "./GroupedEnvironmentsListRootControls";

interface GroupedEnvironmentsListRootProps {
  groupedEnvironments: GroupedEnvironments;
}

export const GroupedEnvironmentsListRoot = ({ groupedEnvironments }: GroupedEnvironmentsListRootProps) => {
  const groupedEnvironmentsListRef = useRef<HTMLUListElement>(null);

  const { instruction, isDragging } = useDraggableGroupedEnvironmentsList({
    ref: groupedEnvironmentsListRef,
    groupWithEnvironments: groupedEnvironments,
  });

  return (
    <Tree.RootNode
      ref={groupedEnvironmentsListRef}
      instruction={instruction}
      combineInstruction={instruction}
      className={cn("cursor-pointer")}
      isDragging={isDragging}
    >
      <Tree.RootNodeHeader isActive={false} className="cursor-pointer">
        <GroupedEnvironmentsListRootControls groupedEnvironments={groupedEnvironments} />
      </Tree.RootNodeHeader>

      {groupedEnvironments.expanded && <GroupedEnvironmentsListChildren groupedEnvironments={groupedEnvironments} />}
    </Tree.RootNode>
  );
};
