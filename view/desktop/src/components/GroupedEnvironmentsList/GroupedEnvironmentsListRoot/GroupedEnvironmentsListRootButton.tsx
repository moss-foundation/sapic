import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { GroupedWithEnvironment } from "../types";

interface GroupedEnvironmentsListRootButtonProps {
  showChildren: boolean;
  setShowChildren: (showChildren: boolean) => void;
  groupedWithEnvironments: GroupedWithEnvironment;
}

export const GroupedEnvironmentsListRootButton = ({
  showChildren,
  setShowChildren,
  groupedWithEnvironments,
}: GroupedEnvironmentsListRootButtonProps) => {
  return (
    <div className="z-10 flex items-center gap-2 overflow-hidden">
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
    </div>
  );
};
