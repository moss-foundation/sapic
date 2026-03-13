import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { ResourceIcon } from "../../../ResourceIcon";
import { NODE_OFFSET } from "../../constants";
import { ResourceNode } from "../types";

interface ResourceNodeAddFormProps {
  depth: number;
  isAddingFolderNode: boolean;
  restrictedNames?: string[];
  onNodeAddCallback?: (node: ResourceNode) => void;
  handleAddFormSubmit: (name: string) => void;
  handleAddFormCancel: () => void;
}

const ResourceNodeAddForm = ({
  depth,
  isAddingFolderNode,
  restrictedNames,
  handleAddFormSubmit,
  handleAddFormCancel,
}: ResourceNodeAddFormProps) => {
  const nodePaddingLeftForAddForm = depth * NODE_OFFSET;

  return (
    <div style={{ paddingLeft: nodePaddingLeftForAddForm }} className="flex w-full min-w-0 items-center gap-1.5">
      <Icon icon="ChevronRight" className={cn("size-[20px] shrink-0 opacity-0")} />
      <ResourceIcon
        resource={{
          id: "Placeholder_AddingNodeId",
          name: "Placeholder_AddingNodeName",
          kind: isAddingFolderNode ? "Dir" : "Item",
          class: "endpoint",
          order: undefined,
          expanded: false,
          path: {
            segments: [],
            raw: "",
          },
          protocol: "Get",
        }}
        className={cn("ml-auto size-[18px]", {
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

export default ResourceNodeAddForm;
