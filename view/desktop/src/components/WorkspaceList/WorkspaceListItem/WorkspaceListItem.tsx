import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { useWorkspaceListItemRenamingForm } from "./hooks/useWorkspaceListItemRenamingForm";
import { WorkspaceListItemActions } from "./WorkspaceListItemActions";
import { WorkspaceListItemButton } from "./WorkspaceListItemButton";
import { WorkspaceListItemIndicator } from "./WorkspaceListItemIndicator";
import { WorkspaceListItemRenamingForm } from "./WorkspaceListItemRenamingForm";

export const WorkspaceListItem = ({ environment }: { environment: StreamEnvironmentsEvent }) => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const { isEditing, setIsEditing, handleRename, handleCancel } = useWorkspaceListItemRenamingForm({ environment });

  const onClick = () => {
    addOrFocusPanel({
      id: "workspace-environments",
      component: "Default",
    });
  };

  return (
    <div className="group/WorkspaceListItem relative flex min-h-[30px] w-full items-center justify-between px-2 py-1">
      <WorkspaceListItemIndicator />

      {isEditing ? (
        <WorkspaceListItemRenamingForm onSubmit={handleRename} onCancel={handleCancel} currentName={environment.name} />
      ) : (
        <WorkspaceListItemButton environment={environment} onClick={onClick} />
      )}
      <WorkspaceListItemActions environment={environment} setIsEditing={setIsEditing} />
    </div>
  );
};
