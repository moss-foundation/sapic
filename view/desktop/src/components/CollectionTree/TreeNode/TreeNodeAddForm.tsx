import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { DebugCollectionIconPlaceholder } from "../DebugCollectionIconPlaceholder";
import { NodeAddForm } from "../NodeAddForm";
import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

interface TreeNodeAddFormProps {
  depth: number;
  isAddingFolderNode: boolean;
  restrictedNames?: (string | number)[];
  onNodeAddCallback?: (node: TreeCollectionNode) => void;
  handleAddFormSubmit: (name: string) => void;
  handleAddFormCancel: () => void;
}

const TreeNodeAddForm = ({
  depth,
  isAddingFolderNode,
  restrictedNames,
  handleAddFormSubmit,
  handleAddFormCancel,
}: TreeNodeAddFormProps) => {
  const { nodeOffset } = useContext(TreeContext);
  const nodePaddingLeftForAddForm = (depth + 1) * nodeOffset;

  return (
    <div style={{ paddingLeft: nodePaddingLeftForAddForm }} className="flex w-full min-w-0 items-center gap-1">
      <Icon icon="ChevronRight" className={cn("opacity-0")} />
      <DebugCollectionIconPlaceholder
        type={isAddingFolderNode ? "Dir" : "Item"}
        protocol={undefined}
        className={cn("ml-auto", {
          "opacity-0": !isAddingFolderNode,
        })}
      />
      <NodeAddForm onSubmit={handleAddFormSubmit} onCancel={handleAddFormCancel} restrictedNames={restrictedNames} />
    </div>
  );
};

export default TreeNodeAddForm;
