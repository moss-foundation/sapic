import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { useWorkspacesListItemRenamingForm } from "./hooks/useWorkspacesListItemRenamingForm";
import { WorkspacesListItemActions } from "./WorkspacesListItemActions";
import { WorkspacesListItemButton } from "./WorkspacesListItemButton";
import { WorkspacesListItemIndicator } from "./WorkspacesListItemIndicator";
import { WorkspacesListItemRenamingForm } from "./WorkspacesListItemRenamingForm";

interface WorkspacesListItemProps {
  environment: StreamEnvironmentsEvent;
}

export const WorkspacesListItem = ({ environment }: WorkspacesListItemProps) => {
  const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

  const { isEditing, setIsEditing, handleRename, handleCancel } = useWorkspacesListItemRenamingForm({ environment });

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

  return (
    <div
      className="group/WorkspaceListItem relative flex min-h-[30px] w-full cursor-pointer items-center justify-between px-4 py-1"
      onClick={onClick}
      role="button"
      tabIndex={0}
    >
      <WorkspacesListItemIndicator isActive={isActive} />

      {isEditing ? (
        <WorkspacesListItemRenamingForm
          onSubmit={handleRename}
          onCancel={handleCancel}
          currentName={environment.name}
        />
      ) : (
        <WorkspacesListItemButton environment={environment} />
      )}
      <WorkspacesListItemActions environment={environment} setIsEditing={setIsEditing} />
    </div>
  );
};
