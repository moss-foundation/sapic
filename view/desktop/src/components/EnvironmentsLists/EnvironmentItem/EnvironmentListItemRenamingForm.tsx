import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils/cn";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface EnvironmentListItemRenamingFormProps {
  handleRename: (name: string) => void;
  handleCancel: () => void;
  environment: StreamEnvironmentsEvent;
  restrictedNames: string[];
  className?: string;
}

export const EnvironmentListItemRenamingForm = ({
  className,
  handleRename,
  handleCancel,
  environment,
  restrictedNames,
}: EnvironmentListItemRenamingFormProps) => {
  return (
    <Tree.NodeControls className={cn("min-h-[22px] py-1", className)}>
      <Tree.NodeTriggers>
        <div className="flex h-5 shrink-0 items-center justify-start">
          <Icon icon="Environment" />
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
