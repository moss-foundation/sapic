import { useContext } from "react";

import TestMossImage from "../../../assets/images/TestMossImage.webp";
import { NodeRenamingForm } from "../NodeRenamingForm";
import { TreeContext } from "../Tree";
import { TreeNodeProps } from "../types";

interface TreeRootNodeRenameFormProps {
  node: TreeNodeProps;
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
}

export const TreeRootNodeRenameForm = ({
  node,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}: TreeRootNodeRenameFormProps) => {
  const { onRootRenameCallback } = useContext(TreeContext);

  return (
    <div className="flex grow cursor-pointer items-center gap-1.5">
      {/* TODO: Replace with the actual image and don't forget to remove image from assets */}
      <div className="flex size-5 shrink-0 items-center justify-center rounded outline-1 outline-(--moss-border-color)">
        <img src={TestMossImage} className="h-full w-full" />
      </div>
      <NodeRenamingForm
        onSubmit={(newName) => {
          handleRenamingFormSubmit(newName);
          onRootRenameCallback?.({ ...node, id: newName });
        }}
        onCancel={handleRenamingFormCancel}
        currentName={node.id}
      />
    </div>
  );
};
