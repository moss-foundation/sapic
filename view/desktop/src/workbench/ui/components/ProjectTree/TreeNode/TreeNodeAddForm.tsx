import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { ResourceIcon } from "../../ResourceIcon";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeNode } from "../types";

interface TreeNodeAddFormProps {
  depth: number;
  isAddingFolderNode: boolean;
  restrictedNames?: string[];
  onNodeAddCallback?: (node: ProjectTreeNode) => void;
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
  const { nodeOffset } = useContext(ProjectTreeContext);

  const nodePaddingLeftForAddForm = (depth + 2) * nodeOffset;

  return (
    <div style={{ paddingLeft: nodePaddingLeftForAddForm }} className="flex w-full min-w-0 items-center gap-1.5">
      <Icon icon="ChevronRight" className={cn("shrink-0 opacity-0")} />
      <ResourceIcon
        resource={{
          id: "Placeholder_AddingNodeId",
          name: "Placeholder_AddingNodeName",
          kind: isAddingFolderNode ? "Dir" : "Item",
          protocol: undefined,
          expanded: true,
          order: Infinity,
          class: "endpoint",
          childNodes: [],
          path: {
            raw: "",
            segments: [],
          },
        }}
        className={cn("ml-auto", {
          "opacity-0": !isAddingFolderNode,
        })}
      />
      <Tree.NodeAddForm
        onSubmit={handleAddFormSubmit}
        onCancel={handleAddFormCancel}
        restrictedNames={restrictedNames}
      />
    </div>
  );
};

export default TreeNodeAddForm;
