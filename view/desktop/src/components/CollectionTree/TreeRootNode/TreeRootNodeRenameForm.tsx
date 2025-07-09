import { useContext } from "react";

import { NodeRenamingForm } from "../NodeRenamingForm";
import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";

interface TreeRootNodeRenameFormProps {
  node: TreeCollectionRootNode;
  restrictedNames?: (string | number)[];
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
}

export const TreeRootNodeRenameForm = ({
  node,
  restrictedNames,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}: TreeRootNodeRenameFormProps) => {
  const { picturePath } = useContext(TreeContext);

  return (
    <div className="flex grow cursor-pointer items-center gap-1.5">
      <div className="flex size-5 shrink-0 items-center justify-center rounded outline-1 outline-(--moss-border-color)">
        {picturePath && <img src={picturePath} className="h-full w-full" />}
      </div>

      <NodeRenamingForm
        onSubmit={(name) => {
          handleRenamingFormSubmit(name);
        }}
        onCancel={handleRenamingFormCancel}
        currentName={node.name}
        restrictedNames={restrictedNames}
      />
    </div>
  );
};
