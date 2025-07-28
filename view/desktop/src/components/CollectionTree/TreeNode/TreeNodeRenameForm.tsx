import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { NodeRenamingForm } from "../NodeRenamingForm";
import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import { TreeNodeIcon } from "./TreeNodeIcon";

interface TreeNodeRenameFormProps {
  node: TreeCollectionNode;
  depth: number;
  restrictedNames: string[];
  onNodeRenameCallback?: (node: TreeCollectionNode) => void;
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
}

const TreeNodeRenameForm = ({
  node,
  depth,
  restrictedNames,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}: TreeNodeRenameFormProps) => {
  const { nodeOffset } = useContext(TreeContext);
  const nodePaddingLeft = depth * nodeOffset;
  const shouldRenderChildNodes = node.kind === "Dir" && node.expanded;

  return (
    <div className="w-full min-w-0">
      <span className="flex w-full items-center gap-1 py-0.5" style={{ paddingLeft: nodePaddingLeft }}>
        <Icon
          icon="ChevronRight"
          className={cn("text-(--moss-icon-primary-text)", {
            "rotate-90": shouldRenderChildNodes,
            "opacity-0": node.kind !== "Dir",
          })}
        />
        <TreeNodeIcon node={node} isRootNode={false} />
        <NodeRenamingForm
          onSubmit={(newName) => {
            handleRenamingFormSubmit(newName);
          }}
          onCancel={handleRenamingFormCancel}
          restrictedNames={restrictedNames}
          currentName={node.name}
        />
      </span>
    </div>
  );
};

export default TreeNodeRenameForm;
