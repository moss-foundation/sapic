import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { NodeAddForm } from "../NodeAddForm";
import { TestCollectionIcon } from "../TestCollectionIcon";
import { TreeContext } from "../Tree";
import { NodeProps, TreeNodeProps } from "../types";

interface TreeNodeAddFormProps {
  node: TreeNodeProps;
  depth: number;
  isAddingFileNode: boolean;
  isAddingFolderNode: boolean;
  onNodeAddCallback?: (node: TreeNodeProps) => void;
  handleAddFormSubmit: (newNode: NodeProps) => void;
  handleAddFormCancel: () => void;
}

const TreeNodeAddForm = ({
  node,
  depth,
  isAddingFileNode,
  isAddingFolderNode,
  onNodeAddCallback,
  handleAddFormSubmit,
  handleAddFormCancel,
}: TreeNodeAddFormProps) => {
  const { nodeOffset } = useContext(TreeContext);
  const nodePaddingLeftForAddForm = (depth + 1) * nodeOffset;

  return (
    <div style={{ paddingLeft: nodePaddingLeftForAddForm }} className="flex w-full min-w-0 items-center gap-1">
      <Icon icon="ChevronRight" className={cn("opacity-0")} />
      <TestCollectionIcon
        type={node.type}
        className={cn("ml-auto", {
          "opacity-0": isAddingFileNode,
        })}
      />
      <NodeAddForm
        isFolder={isAddingFolderNode}
        restrictedNames={node.childNodes.map((childNode) => childNode.id)}
        onSubmit={(newNode) => {
          handleAddFormSubmit(newNode);
          onNodeAddCallback?.({ ...node, childNodes: [...node.childNodes, newNode] } as TreeNodeProps);
        }}
        onCancel={handleAddFormCancel}
      />
    </div>
  );
};

export default TreeNodeAddForm;
