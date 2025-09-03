import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils/cn";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface GlobalEnvironmentsListItemRenamingFormProps {
  handleRename: (name: string) => void;
  handleCancel: () => void;
  environment: StreamEnvironmentsEvent;
  restrictedNames: string[];
  className?: string;
}

export const GlobalEnvironmentsListItemRenamingForm = ({
  className,
  handleRename,
  handleCancel,
  environment,
  restrictedNames,
}: GlobalEnvironmentsListItemRenamingFormProps) => {
  return (
    <Tree.RootNodeControls className={cn("min-h-[22px]", className)}>
      <Tree.RootNodeTriggers>
        <Icon icon="Environment" />
        <Tree.NodeRenamingForm
          onSubmit={handleRename}
          onCancel={handleCancel}
          currentName={environment.name}
          restrictedNames={restrictedNames}
        />
      </Tree.RootNodeTriggers>
    </Tree.RootNodeControls>
  );
};
