import { useMemo, useRef } from "react";

import { DropIndicator } from "@/components/DropIndicator";
import { useStreamEnvironments } from "@/hooks/environment";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { useDraggableWorkspacesListItem } from "./hooks/useDraggableWorkspacesListItem";
import { useWorkspacesListItemRenamingForm } from "./hooks/useWorkspacesListItemRenamingForm";
import { WorkspacesListItemActions } from "./WorkspacesListItemActions";
import { WorkspacesListItemButton } from "./WorkspacesListItemButton";
import { WorkspacesListItemIndicator } from "./WorkspacesListItemIndicator";
import { WorkspacesListItemRenamingForm } from "./WorkspacesListItemRenamingForm";

interface WorkspacesListItemProps {
  environment: StreamEnvironmentsEvent;
}

export const WorkspacesListItem = ({ environment }: WorkspacesListItemProps) => {
  const workspaceListItemRef = useRef<HTMLDivElement>(null);

  const { data: environments } = useStreamEnvironments();
  const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

  const { isEditing, setIsEditing, handleRename, handleCancel } = useWorkspacesListItemRenamingForm({ environment });

  const { closestEdge } = useDraggableWorkspacesListItem({ ref: workspaceListItemRef, environment });

  const onClick = () => {
    addOrFocusPanel({
      id: environment.id,
      component: "Default",
      title: environment.name,
      params: {
        iconType: "Environment",
      },
    });
  };

  const isActive = activePanelId === environment.id;

  const restrictedNames = useMemo(() => {
    if (!environments) return [];
    return environments.map((environment) => environment.name) ?? [];
  }, [environments]);

  return (
    <div
      ref={workspaceListItemRef}
      className="group/WorkspaceListItem relative flex min-h-[30px] w-full cursor-pointer items-center justify-between py-1 pr-2 pl-2.5"
      onClick={onClick}
      role="button"
      tabIndex={0}
    >
      <WorkspacesListItemIndicator isActive={isActive} />
      {closestEdge && <DropIndicator edge={closestEdge} />}

      {isEditing ? (
        <WorkspacesListItemRenamingForm
          onSubmit={handleRename}
          onCancel={handleCancel}
          currentName={environment.name}
          restrictedNames={restrictedNames}
        />
      ) : (
        <WorkspacesListItemButton environment={environment} />
      )}
      <WorkspacesListItemActions environment={environment} setIsEditing={setIsEditing} />
    </div>
  );
};
