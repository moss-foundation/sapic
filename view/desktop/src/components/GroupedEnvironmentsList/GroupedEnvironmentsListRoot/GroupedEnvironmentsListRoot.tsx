import { useState } from "react";

import { Tree } from "@/lib/ui/Tree";

import { GroupedEnvironmentsListChildren } from "../GroupedEnvironmentsListChildren";
import { GroupedWithEnvironment } from "../types";
import { GroupedEnvironmentsListRootControls } from "./GroupedEnvironmentsListRootControls";

interface GroupedEnvironmentsListRootProps {
  groupedWithEnvironments: GroupedWithEnvironment;
}

export const GroupedEnvironmentsListRoot = ({ groupedWithEnvironments }: GroupedEnvironmentsListRootProps) => {
  const [showChildren, setShowChildren] = useState(true);

  const onClick = () => {
    setShowChildren(!showChildren);
  };

  return (
    <Tree.RootNode isChildDropBlocked={false} instruction={null}>
      <Tree.RootNodeHeader onClick={onClick} isActive={false} className="cursor-pointer">
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
