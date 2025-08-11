import { useState } from "react";

import { ActionMenu } from "@/components";
import ActionButton from "@/components/ActionButton";
import { Icon } from "@/lib/ui";
import { useWorkspaceListStore } from "@/store/workspaceList";
import { cn } from "@/utils";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface WorkspaceListItemActionsProps {
  environment: StreamEnvironmentsEvent;
  setIsEditing: (isEditing: boolean) => void;
}

export const WorkspaceListItemActions = ({ environment, setIsEditing }: WorkspaceListItemActionsProps) => {
  const { setActiveEnvironment, activeEnvironment } = useWorkspaceListStore();
  const [showActionMenu, setShowActionMenu] = useState(false);

  return (
    <div className="z-10 flex items-center gap-2">
      <button className="cursor-pointer" onClick={() => setActiveEnvironment(environment)}>
        <Icon icon={activeEnvironment?.id === environment.id ? "EnvironmentSelectionActive" : "EnvironmentSelection"} />
      </button>

      <ActionMenu.Root onOpenChange={setShowActionMenu} modal={showActionMenu}>
        <ActionMenu.Trigger
          asChild
          className={cn("sr-only group-hover/WorkspaceListItem:not-sr-only", { "not-sr-only": showActionMenu })}
        >
          <ActionButton
            icon="MoreHorizontal"
            className="cursor-pointer"
            customHoverBackground="hover:background-(--moss-gray-10)"
          />
        </ActionMenu.Trigger>

        <ActionMenu.Portal>
          <ActionMenu.Content>
            <ActionMenu.Item onClick={() => setIsEditing(true)}>Edit</ActionMenu.Item>
            <ActionMenu.Item>Delete</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    </div>
  );
};
