import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { NodeAddForm } from "../NodeAddForm";
import { TestCollectionIcon } from "../TestCollectionIcon";
import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

interface TreeNodeAddFormProps {
  node: TreeCollectionNode;
  depth: number;
  isAddingFolderNode: boolean;
  onNodeAddCallback?: (node: TreeCollectionNode) => void;
  handleAddFormSubmit: (name: string) => void;
  handleAddFormCancel: () => void;
}

const TreeNodeAddForm = ({
  node,
  depth,
  isAddingFolderNode,
  handleAddFormSubmit,
  handleAddFormCancel,
}: TreeNodeAddFormProps) => {
  const { nodeOffset } = useContext(TreeContext);
  const nodePaddingLeftForAddForm = (depth + 1) * nodeOffset;

  return (
    <div style={{ paddingLeft: nodePaddingLeftForAddForm }} className="flex w-full min-w-0 items-center gap-1">
      <Icon icon="ChevronRight" className={cn("opacity-0")} />
      <TestCollectionIcon
        type={node.kind}
        className={cn("ml-auto", {
          "opacity-0": !isAddingFolderNode,
        })}
      />
      <NodeAddForm onSubmit={handleAddFormSubmit} onCancel={handleAddFormCancel} />
    </div>
  );
};

export default TreeNodeAddForm;
