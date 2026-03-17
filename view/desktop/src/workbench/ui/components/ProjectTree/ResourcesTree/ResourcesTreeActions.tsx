import { useState } from "react";

import { Tree } from "@/lib/ui/Tree";
import { ActionButton, ActionMenu } from "@/workbench/ui/components";

interface ResourcesTreeActionsProps {
  setIsAddingFileNode: (isAdding: boolean) => void;
  setIsAddingFolderNode: (isAdding: boolean) => void;
}

export const ResourcesTreeActions = ({ setIsAddingFileNode, setIsAddingFolderNode }: ResourcesTreeActionsProps) => {
  const [showDropdown, setShowDropdown] = useState(false);

  return (
    <Tree.ListActions>
      <Tree.ActionsHover forceVisible={showDropdown} invisible>
        <ActionMenu.Root onOpenChange={setShowDropdown} modal={showDropdown}>
          <ActionMenu.Trigger asChild>
            <ActionButton hoverVariant="list" icon="MoreHorizontal" forceHoverStyles={showDropdown} />
          </ActionMenu.Trigger>
          <ActionMenu.Portal>
            <ActionMenu.Content className="z-40" align="center">
              <ActionMenu.Item onClick={() => setIsAddingFileNode(true)} icon="Http">
                Add Resource
              </ActionMenu.Item>
              <ActionMenu.Item onClick={() => setIsAddingFolderNode(true)} icon="Folder">
                Add Folder
              </ActionMenu.Item>
            </ActionMenu.Content>
          </ActionMenu.Portal>
        </ActionMenu.Root>
      </Tree.ActionsHover>
    </Tree.ListActions>
  );
};
