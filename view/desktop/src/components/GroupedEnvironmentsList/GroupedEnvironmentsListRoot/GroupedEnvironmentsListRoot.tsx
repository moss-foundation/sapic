import { useState } from "react";

import { GroupedEnvironmentsListChildren } from "../GroupedEnvironmentsListChildren";
import { GroupedEnvironmentsListItemIndicator } from "../GroupedEnvironmentsListItemIndicator";
import { GroupedWithEnvironment } from "../types";
import { GroupedEnvironmentsListRootActions } from "./GroupedEnvironmentsListRootActions";
import { GroupedEnvironmentsListRootButton } from "./GroupedEnvironmentsListRootButton";

interface GroupedEnvironmentsListRootProps {
  groupedWithEnvironments: GroupedWithEnvironment;
}

export const GroupedEnvironmentsListRoot = ({ groupedWithEnvironments }: GroupedEnvironmentsListRootProps) => {
  const [showChildren, setShowChildren] = useState(true);

  const onClick = () => {
    setShowChildren(!showChildren);
  };

  return (
    <div className="group/GroupedEnvironmentsListRoot flex flex-col">
      <div
        className="group/GroupedEnvironmentsListRootHeader relative flex h-[30px] cursor-pointer items-center justify-between py-2 pr-2 pl-[10px]"
        onClick={onClick}
        role="button"
        tabIndex={0}
      >
        <GroupedEnvironmentsListRootButton
          showChildren={showChildren}
          setShowChildren={setShowChildren}
          groupedWithEnvironments={groupedWithEnvironments}
        />

        <GroupedEnvironmentsListRootActions />

        <GroupedEnvironmentsListItemIndicator />
      </div>

      {showChildren && <GroupedEnvironmentsListChildren groupedWithEnvironments={groupedWithEnvironments} />}
    </div>
  );
};
