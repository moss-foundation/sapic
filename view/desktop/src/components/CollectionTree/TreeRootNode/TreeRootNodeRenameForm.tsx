import { useContext } from "react";

import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";

import { CollectionTreeContext } from "../CollectionTreeContext";
import { NodeRenamingForm } from "../NodeRenamingForm";
import { TreeCollectionRootNode } from "../types";

interface TreeRootNodeRenameFormProps {
  node: TreeCollectionRootNode;
  shouldRenderChildNodes: boolean;
  restrictedNames: string[];
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
}

export const TreeRootNodeRenameForm = ({
  node,
  shouldRenderChildNodes,
  restrictedNames,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}: TreeRootNodeRenameFormProps) => {
  const { iconPath } = useContext(CollectionTreeContext);

  return (
    <div className="flex grow cursor-pointer items-center gap-1.5">
      <div className="flex size-5 shrink-0 items-center justify-center rounded">
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

      <NodeRenamingForm
        onSubmit={handleRenamingFormSubmit}
        onCancel={handleRenamingFormCancel}
        currentName={node.name}
        restrictedNames={restrictedNames}
      />
    </div>
  );
};
