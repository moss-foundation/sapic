import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { EntryIcon } from "../../EntryIcon";
import { NodeRenamingForm } from "../NodeRenamingForm";
import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

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
  const { nodeOffset, treePaddingLeft } = useContext(TreeContext);
  const nodePaddingLeft = depth * nodeOffset + treePaddingLeft;
  const shouldRenderChildNodes = node.kind === "Dir" && node.expanded;

  return (
    <div className="w-full min-w-0">
      <span className="flex w-full items-center gap-1 py-1" style={{ paddingLeft: nodePaddingLeft }}>
        <Icon
          icon="ChevronRight"
          className={cn("size-5 text-(--moss-icon-primary-text)", {
            "rotate-90": shouldRenderChildNodes,
            "opacity-0": node.kind !== "Dir",
          })}
        />

        <EntryIcon entry={node} />

        <NodeRenamingForm
          onSubmit={handleRenamingFormSubmit}
          onCancel={handleRenamingFormCancel}
          restrictedNames={restrictedNames}
          currentName={node.name}
        />
      </span>
    </div>
  );
};

export default TreeNodeRenameForm;
