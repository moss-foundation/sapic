import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { NodeRenamingForm } from "../NodeRenamingForm";
import { TestCollectionIcon } from "../TestCollectionIcon";
import { TreeContext } from "../Tree";
import { TreeNodeProps } from "../types";

interface TreeNodeRenameFormProps {
  node: TreeNodeProps;
  depth: number;
  parentNode: TreeNodeProps;
  onNodeRenameCallback?: (node: TreeNodeProps) => void;
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
}

const TreeNodeRenameForm = ({
  node,
  depth,
  parentNode,
  onNodeRenameCallback,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}: TreeNodeRenameFormProps) => {
  const { nodeOffset, searchInput } = useContext(TreeContext);
  const nodePaddingLeft = depth * nodeOffset;
  const shouldRenderChildNodes = !!searchInput || (!searchInput && node.isFolder && node.isExpanded);

  return (
    <div className="w-full min-w-0">
      <span className="flex w-full items-center gap-1 py-0.5" style={{ paddingLeft: nodePaddingLeft }}>
        <Icon
          icon="ChevronRight"
          className={cn("text-(--moss-icon-primary-text)", {
            "rotate-90": shouldRenderChildNodes,
            "opacity-0": !node.isFolder,
          })}
        />
        <TestCollectionIcon type={node.type} />
        <NodeRenamingForm
          onSubmit={(newName) => {
            handleRenamingFormSubmit(newName);
            onNodeRenameCallback?.({ ...node, id: newName });
          }}
          onCancel={handleRenamingFormCancel}
          restrictedNames={parentNode.childNodes.map((childNode) => childNode.id)}
          currentName={node.id}
        />
      </span>
    </div>
  );
};

export default TreeNodeRenameForm;
