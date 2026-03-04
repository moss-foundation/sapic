import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils/cn";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";

interface EnvironmentItemRenamingFormProps {
  handleRename: (name: string) => void;
  handleCancel: () => void;
  environment: EnvironmentSummary;
  restrictedNames: string[];
  className?: string;
  type: ENVIRONMENT_ITEM_DRAG_TYPE;
  offsetLeft?: number;
}

export const EnvironmentItemRenamingForm = ({
  className,
  handleRename,
  handleCancel,
  environment,
  restrictedNames,
  type,
  offsetLeft,
}: EnvironmentItemRenamingFormProps) => {
  return (
    <Tree.NodeDetails
      depth={type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ? 0 : 1}
      className={cn("pb-[4px] pt-[5px]", className)}
    >
      <Tree.NodeTriggers style={{ paddingLeft: offsetLeft }}>
        <Tree.Decorator />

        <Tree.NodeRenamingForm
          onSubmit={handleRename}
          onCancel={handleCancel}
          currentName={environment.name}
          restrictedNames={restrictedNames}
        />
      </Tree.NodeTriggers>
    </Tree.NodeDetails>
  );
};
