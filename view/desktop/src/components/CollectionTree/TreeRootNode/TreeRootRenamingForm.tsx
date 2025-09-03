import { Tree } from "@/lib/ui/Tree";

import { TreeCollectionRootNode } from "../types";

interface TreeRootRenamingFormProps {
  node: TreeCollectionRootNode;
  shouldRenderChildNodes: boolean;
  restrictedNames: string[];
  handleRenamingFormSubmit: (name: string) => void;
  handleRenamingFormCancel: () => void;
}

export const TreeRootRenamingForm = ({
  node,
  restrictedNames,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}: TreeRootRenamingFormProps) => {
  return (
    <Tree.NodeRenamingForm
      currentName={node.name}
      restrictedNames={restrictedNames}
      onSubmit={handleRenamingFormSubmit}
      onCancel={handleRenamingFormCancel}
    />
  );
};
