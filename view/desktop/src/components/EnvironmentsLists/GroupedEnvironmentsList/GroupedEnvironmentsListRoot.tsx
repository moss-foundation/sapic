import { useRef, useState } from "react";

import { Tree } from "@/lib/ui/Tree";

import { GroupedEnvironments } from "../types";
import { GroupedEnvironmentsListChildren } from "./GroupedEnvironmentsListChildren";
import { GroupedEnvironmentsListRootControls } from "./GroupedEnvironmentsListRootControls";

interface GroupedEnvironmentsListRootProps {
  groupedEnvironments: GroupedEnvironments;
}

export const GroupedEnvironmentsListRoot = ({ groupedEnvironments }: GroupedEnvironmentsListRootProps) => {
  const [showChildren, setShowChildren] = useState(true);
  const groupedEnvironmentsListRef = useRef<HTMLLIElement>(null);

  const onClick = () => {
    setShowChildren(!showChildren);
  };

  // const { instruction, isDragging } = useDraggableGroupedEnvironmentsList({
  //   ref: groupedWithEnvironmentsListRef,
  //   groupWithEnvironments: groupedEnvironments,
  // });

  return (
    <Tree.RootNode
    //instruction={instruction}
    //className={cn("cursor-pointer", isDragging && "opacity-50")}
    >
      <Tree.RootNodeHeader
        ref={groupedEnvironmentsListRef}
        onClick={onClick}
        isActive={false}
        className="cursor-pointer"
      >
        <GroupedEnvironmentsListRootControls
          showChildren={showChildren}
          setShowChildren={setShowChildren}
          groupedWithEnvironments={groupedEnvironments}
        />
      </Tree.RootNodeHeader>

      {showChildren && <GroupedEnvironmentsListChildren groupedEnvironments={groupedEnvironments} />}
    </Tree.RootNode>
  );
};
