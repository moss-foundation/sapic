import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils/cn";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "./constants";

interface EnvironmentItemRenamingFormProps {
  handleRename: (name: string) => void;
  handleCancel: () => void;
  environment: EnvironmentSummary;
  restrictedNames: string[];
  className?: string;
  type: ENVIRONMENT_ITEM_DRAG_TYPE;
}

export const EnvironmentItemRenamingForm = ({
  className,
  handleRename,
  handleCancel,
  environment,
  restrictedNames,
  type,
}: EnvironmentItemRenamingFormProps) => {
  return (
    <Tree.NodeControls
      depth={type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ? 0 : 1}
      className={cn("min-h-[22px] py-1", className)}
    >
      <Tree.NodeTriggers>
        <div className="flex h-5 shrink-0 items-center justify-start">
          <Icon icon={type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ? "ProjectEnvironment" : "WorkspaceEnvironment"} />
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
