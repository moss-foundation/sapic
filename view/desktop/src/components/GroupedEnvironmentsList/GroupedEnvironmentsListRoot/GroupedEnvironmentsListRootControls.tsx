import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { GroupedWithEnvironment } from "../types";
import { GroupedEnvironmentsListRootActions } from "./GroupedEnvironmentsListRootActions";

interface GroupedEnvironmentsListRootControlsProps {
  showChildren: boolean;
  setShowChildren: (showChildren: boolean) => void;
  groupedWithEnvironments: GroupedWithEnvironment;
}

export const GroupedEnvironmentsListRootControls = ({
  showChildren,
  setShowChildren,
  groupedWithEnvironments,
}: GroupedEnvironmentsListRootControlsProps) => {
  return (
    <Tree.RootNodeControls>
      <Tree.RootNodeTriggers>
        <button
          onClick={(e) => {
            e.stopPropagation();
            setShowChildren(!showChildren);
          }}
          className="hover:background-(--moss-icon-primary-background-hover) flex size-4 cursor-pointer items-center justify-center rounded-full"
        >
          <Icon icon="ChevronRight" className={cn(showChildren && "rotate-90")} />
        </button>

        <div className="truncate font-medium">{groupedWithEnvironments.name}</div>
      </Tree.RootNodeTriggers>

      <Tree.RootNodeActions>
        <GroupedEnvironmentsListRootActions />
      </Tree.RootNodeActions>
    </Tree.RootNodeControls>
  );
};
