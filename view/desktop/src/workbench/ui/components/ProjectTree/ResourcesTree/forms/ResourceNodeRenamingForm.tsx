import { HTMLAttributes, useContext } from "react";

import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { ResourceIcon } from "../../../ResourceIcon";
import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ResourceNode } from "../../types";

interface TreeNodeRenamingFormProps extends HTMLAttributes<HTMLDivElement> {
  node: ResourceNode;
  depth: number;
  restrictedNames: string[];
  onNodeRenameCallback?: (node: ResourceNode) => void;
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
  const { nodeOffset, treePaddingLeft } = useContext(ProjectTreeContext);
  const nodePaddingLeft = depth * treePaddingLeft;
  const shouldRenderChildNodes = node.kind === "Dir" && node.expanded;

  return (
    <div className={cn("w-full min-w-0", className)} {...props}>
      <div className="flex w-full items-center gap-1.5 py-1" style={{ paddingLeft: nodePaddingLeft }}>
        <div className="flex size-5 shrink-0 items-center justify-center">
          <Icon
            icon="ChevronRight"
            className={cn({
              "rotate-90": shouldRenderChildNodes,
              "opacity-0": node.kind !== "Dir",
            })}
          />
        </div>

        <ResourceIcon resource={node} />

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
