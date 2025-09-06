import { useRef } from "react";

import { Tree } from "@/lib/ui/Tree";

import { GroupedEnvironments } from "../types";
import { GroupedEnvironmentsListChildren } from "./GroupedEnvironmentsListChildren";
import { GroupedEnvironmentsListRootControls } from "./GroupedEnvironmentsListRootControls";

interface GroupedEnvironmentsListRootProps {
  groupedEnvironments: GroupedEnvironments;
}

export const GroupedEnvironmentsListRoot = ({ groupedEnvironments }: GroupedEnvironmentsListRootProps) => {
  const groupedEnvironmentsListRef = useRef<HTMLLIElement>(null);

  // const { instruction, isDragging } = useDraggableGroupedEnvironmentsList({
  //   ref: groupedWithEnvironmentsListRef,
  //   groupWithEnvironments: groupedEnvironments,
  // });

  return (
    <Tree.RootNode
    //instruction={instruction}
    //className={cn("cursor-pointer", isDragging && "opacity-50")}
    >
      <Tree.RootNodeHeader ref={groupedEnvironmentsListRef} isActive={false} className="cursor-pointer">
        <GroupedEnvironmentsListRootControls groupedEnvironments={groupedEnvironments} />
      </Tree.RootNodeHeader>

      {groupedEnvironments.expanded && <GroupedEnvironmentsListChildren groupedEnvironments={groupedEnvironments} />}
    </Tree.RootNode>
  );
};
