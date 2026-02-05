import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils/cn";

import { EnvironmentListType } from "../types";

interface EnvironmentListItemRenamingFormProps {
  handleRename: (name: string) => void;
  handleCancel: () => void;
  environment: EnvironmentSummary;
  restrictedNames: string[];
  className?: string;
  type: EnvironmentListType;
}

export const EnvironmentListItemRenamingForm = ({
  className,
  handleRename,
  handleCancel,
  environment,
  restrictedNames,
  type,
}: EnvironmentListItemRenamingFormProps) => {
  return (
    <Tree.NodeControls depth={type === "GlobalEnvironmentItem" ? 0 : 1} className={cn("min-h-[22px] py-1", className)}>
      <Tree.NodeTriggers>
        <div className="flex h-5 shrink-0 items-center justify-start">
          <Icon icon={type === "GlobalEnvironmentItem" ? "Environment" : "GroupedEnvironment"} />
        </div>
        <Tree.NodeRenamingForm
          onSubmit={handleRename}
          onCancel={handleCancel}
          currentName={environment.name}
          restrictedNames={restrictedNames}
        />
      </Tree.NodeTriggers>
    </Tree.NodeControls>
  );
};
