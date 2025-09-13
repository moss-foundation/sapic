import { HTMLAttributes, useContext } from "react";

import { Icon } from "@/lib/ui/Icon";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";

interface TreeRootNodeRenamingFormProps extends HTMLAttributes<HTMLDivElement> {
  node: ProjectTreeRootNode;
  shouldRenderChildNodes: boolean;
  restrictedNames: string[];
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
}

export const TreeRootNodeRenamingForm = ({
  node,
  shouldRenderChildNodes,
  restrictedNames,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
  className,
  ...props
}: TreeRootNodeRenamingFormProps) => {
  const { iconPath } = useContext(ProjectTreeContext);

  return (
    <div className={cn("flex grow cursor-pointer items-center gap-1.5 py-px", className)} {...props}>
      <div className="flex h-5 shrink-0 items-center justify-center rounded">
        {iconPath ? (
          <img src={iconPath} className="h-full w-full" />
        ) : (
          <span className="flex size-5 shrink-0 items-center justify-center">
            <button className="flex cursor-pointer items-center justify-center rounded-full">
              <Icon icon="ChevronRight" className={cn({ "rotate-90": shouldRenderChildNodes })} />
            </button>
          </span>
        )}
      </div>

      <Tree.NodeRenamingForm
        onSubmit={handleRenamingFormSubmit}
        onCancel={handleRenamingFormCancel}
        currentName={node.name}
        restrictedNames={restrictedNames}
      />
    </div>
  );
};
