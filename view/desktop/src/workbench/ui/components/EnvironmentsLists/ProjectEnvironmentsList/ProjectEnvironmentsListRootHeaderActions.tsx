import { useState } from "react";

import { Tree } from "@/lib/ui/Tree";

import { ActionMenu } from "../..";
import { ActionButton } from "../../ActionButton";

interface ProjectEnvironmentsListRootHeaderActionsProps {
  setIsAddingProjectEnvironment: (isAddingProjectEnvironment: boolean) => void;
}

export const ProjectEnvironmentsListRootHeaderActions = ({
  setIsAddingProjectEnvironment,
}: ProjectEnvironmentsListRootHeaderActionsProps) => {
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
              <ActionMenu.Item onClick={() => setIsAddingProjectEnvironment(true)}>Add Environment</ActionMenu.Item>
            </ActionMenu.Content>
          </ActionMenu.Portal>
        </ActionMenu.Root>
      </Tree.ActionsHover>
    </Tree.ListActions>
  );
};
