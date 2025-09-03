import { HTMLAttributes, useContext } from "react";

import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { EntryIcon } from "../../EntryIcon";
import { CollectionTreeContext } from "../CollectionTreeContext";
import { TreeCollectionNode } from "../types";

interface TreeNodeRenamingFormProps extends HTMLAttributes<HTMLDivElement> {
  node: TreeCollectionNode;
  depth: number;
  restrictedNames: string[];
  onNodeRenameCallback?: (node: TreeCollectionNode) => void;
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
  className?: string;
}

const TreeNodeRenamingForm = ({
  node,
  depth,
  restrictedNames,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
  className,
  ...props
}: TreeNodeRenamingFormProps) => {
  const { nodeOffset, treePaddingLeft } = useContext(CollectionTreeContext);
  const nodePaddingLeft = depth * nodeOffset + treePaddingLeft;
  const shouldRenderChildNodes = node.kind === "Dir" && node.expanded;

  return (
    <div className={cn("w-full min-w-0", className)} {...props}>
      <div className="flex w-full items-center gap-1 py-1" style={{ paddingLeft: nodePaddingLeft }}>
        <div className="flex size-5 shrink-0 items-center justify-center">
          <Icon
            icon="ChevronRight"
            className={cn("text-(--moss-icon-primary-text)", {
              "rotate-90": shouldRenderChildNodes,
              "opacity-0": node.kind !== "Dir",
            })}
          />
        </div>

        <EntryIcon entry={node} />

        <Tree.NodeRenamingForm
          onSubmit={handleRenamingFormSubmit}
          onCancel={handleRenamingFormCancel}
          restrictedNames={restrictedNames}
          currentName={node.name}
        />
      </div>
    </div>
  );
};

export default TreeNodeRenamingForm;
