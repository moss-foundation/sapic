import { useState } from "react";

import { useDeleteWorkspace } from "@/adapters/tanstackQuery/workspace/useDeleteWorkspace";
import { useModal } from "@/hooks";
import { useRenameWorkspace } from "@/hooks/useRenameWorkspace";
import { useActiveWorkspace } from "@/hooks/workspace/derived/useActiveWorkspace";
import { ConfirmationModal } from "@/workbench/ui/components/Modals/ConfirmationModal";
import { PageHeader } from "@/workbench/ui/components/PageView/PageHeader";
import { PageView } from "@/workbench/ui/components/PageView/PageView";
import { DefaultViewProps } from "@/workbench/ui/parts/TabbedPane/types";

import { WorkspaceDangerZoneSection } from "./WorkspaceDangerZoneSection";
import { WorkspaceDataSection } from "./WorkspaceDataSection";
import { WorkspaceNameSection } from "./WorkspaceNameSection";
import { WorkspaceStartupSection } from "./WorkspaceStartupSection";

export type WorkspaceSettingsViewProps = DefaultViewProps;

export const WorkspaceSettingsView = ({ ...props }: WorkspaceSettingsViewProps) => {
  const { hasActiveWorkspace, activeWorkspace } = useActiveWorkspace();

  const { mutate: deleteWorkspace, isPending: isDeleting } = useDeleteWorkspace();

  const [name, setName] = useState(activeWorkspace?.name || "");
  const [reopenOnNextSession, setReopenOnNextSession] = useState(false);
  const [openPreviousWindows, setOpenPreviousWindows] = useState(false);

  const {
    openModal: openDeleteWorkspaceModal,
    closeModal: closeDeleteWorkspaceModal,
    showModal: isDeleteWorkspaceModalOpen,
  } = useModal();

  const { isRenamingWorkspace, setIsRenamingWorkspace, handleRenamingWorkspaceSubmit, handleRenamingWorkspaceCancel } =
    useRenameWorkspace(activeWorkspace);

  const handleBlur = () => {
    handleRenamingWorkspaceSubmit(name);
  };

  const handleDeleteClick = () => {
    openDeleteWorkspaceModal();
  };

  const handleDeleteWorkspace = () => {
    if (activeWorkspace) {
      deleteWorkspace(
        { id: activeWorkspace.id },
        {
          onSuccess: () => {
            closeDeleteWorkspaceModal();
          },
          onError: (error) => {
            console.error("Failed to delete workspace:", error.message);
            closeDeleteWorkspaceModal();
          },
        }
      );
    }
  };

  if (!hasActiveWorkspace) {
    return (
      <div className="text-(--moss-primary-foreground) flex h-full items-center justify-center">
        <div className="text-center">
          <h2 className="text-lg font-semibold">No Active Workspace</h2>
          <p className="text-sm">Please select a workspace to view its settings.</p>
        </div>
      </div>
    );
  }

  return (
    <PageView>
      <PageHeader
        icon="Workspace"
        title={activeWorkspace?.name}
        onTitleChange={handleRenamingWorkspaceSubmit}
        disableTitleChange={false}
        isRenamingTitle={isRenamingWorkspace}
        setIsRenamingTitle={setIsRenamingWorkspace}
        handleRenamingFormCancel={handleRenamingWorkspaceCancel}
        {...props}
      />

      <div className="flex h-full justify-center">
        <div className="w-full max-w-2xl space-y-6 px-6 py-5">
          <WorkspaceNameSection
            name={name}
            setName={setName}
            onSave={() => handleRenamingWorkspaceSubmit(name)}
            onBlur={handleBlur}
          />

          <WorkspaceStartupSection
            reopenOnNextSession={reopenOnNextSession}
            setReopenOnNextSession={setReopenOnNextSession}
            openPreviousWindows={openPreviousWindows}
            setOpenPreviousWindows={setOpenPreviousWindows}
          />

          <WorkspaceDataSection />

          <WorkspaceDangerZoneSection onDeleteClick={handleDeleteClick} />
        </div>
      </div>

      {isDeleteWorkspaceModalOpen && (
        <ConfirmationModal
          showModal={isDeleteWorkspaceModalOpen}
          closeModal={closeDeleteWorkspaceModal}
          title="Delete"
          message={`Delete "${activeWorkspace?.name}"?`}
          description="This will delete the monitors, scheduled runs and integrations and deactivate the mock servers associated with projects in the workspace."
          confirmLabel={isDeleting ? "Deleting..." : "Delete"}
          cancelLabel="Close"
          onConfirm={handleDeleteWorkspace}
          variant="danger"
          loading={isDeleting}
        />
      )}
    </PageView>
  );
};
