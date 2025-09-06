import { useRef } from "react";

import { Tree } from "@/lib/ui/Tree";

import { GroupedEnvironments } from "../types";
import { GroupedEnvironmentsListChildren } from "./GroupedEnvironmentsListChildren";
import { GroupedEnvironmentsListRootControls } from "./GroupedEnvironmentsListRootControls";

interface GroupedEnvironmentsListRootProps {
  groupedEnvironments: GroupedEnvironments;
}

export const GroupedEnvironmentsListRoot = ({ groupedEnvironments }: GroupedEnvironmentsListRootProps) => {
  const groupedEnvironmentsListRef = useRef<HTMLUListElement>(null);

  // const { isChildDropBlocked, instruction } = useDraggableGroupedEnvironmentsList({
  //   ref: groupedEnvironmentsListRef,
  //   groupWithEnvironments: groupedEnvironments,
  // });

  return (
    <Tree.RootNode
      ref={groupedEnvironmentsListRef}
      // instruction={instruction}
      // isChildDropBlocked={isChildDropBlocked}
      //className={cn("cursor-pointer", isDragging && "opacity-50")}
    >
      <Tree.RootNodeHeader isActive={false} className="cursor-pointer">
        <GroupedEnvironmentsListRootControls groupedEnvironments={groupedEnvironments} />
      </Tree.RootNodeHeader>

      {groupedEnvironments.expanded && <GroupedEnvironmentsListChildren groupedEnvironments={groupedEnvironments} />}
    </Tree.RootNode>
  );
};
