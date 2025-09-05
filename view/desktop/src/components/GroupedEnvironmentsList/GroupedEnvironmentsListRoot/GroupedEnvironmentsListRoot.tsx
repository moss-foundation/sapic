import { useRef, useState } from "react";

import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { GroupedEnvironmentsListChildren } from "../GroupedEnvironmentsListChildren";
import { useDraggableGroupedEnvironmentsList } from "../hooks/useDraggableGroupedEnvironments";
import { GroupedWithEnvironment } from "../types";
import { GroupedEnvironmentsListRootControls } from "./GroupedEnvironmentsListRootControls";

interface GroupedEnvironmentsListRootProps {
  groupedWithEnvironments: GroupedWithEnvironment;
}

export const GroupedEnvironmentsListRoot = ({ groupedWithEnvironments }: GroupedEnvironmentsListRootProps) => {
  const [showChildren, setShowChildren] = useState(true);
  const groupedWithEnvironmentsListRef = useRef<HTMLLIElement>(null);

  const onClick = () => {
    setShowChildren(!showChildren);
  };

  const { instruction, isDragging } = useDraggableGroupedEnvironmentsList({
    ref: groupedWithEnvironmentsListRef,
    groupWithEnvironments: groupedWithEnvironments,
  });

  return (
    <Tree.RootNode instruction={instruction} className={cn("cursor-pointer", isDragging && "opacity-50")}>
      <Tree.RootNodeHeader
        ref={groupedWithEnvironmentsListRef}
        onClick={onClick}
        isActive={false}
        className="cursor-pointer"
      >
        <GroupedEnvironmentsListRootControls
          showChildren={showChildren}
          setShowChildren={setShowChildren}
          groupedWithEnvironments={groupedWithEnvironments}
        />
      </Tree.RootNodeHeader>

      {showChildren && <GroupedEnvironmentsListChildren groupedWithEnvironments={groupedWithEnvironments} />}
    </Tree.RootNode>
  );
};
