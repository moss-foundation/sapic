import { useEffect, useState } from "react";

import { ConfirmationModal } from "@/components/Modals/ConfirmationModal";
import { PageHeader } from "@/components/PageView/PageHeader";
import { PageView } from "@/components/PageView/PageView";
import { useModal } from "@/hooks";
import { useRenameWorkspace } from "@/hooks/useRenameWorkspace";
import { useDeleteWorkspace } from "@/hooks/workbench/useDeleteWorkspace";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";
import { IDockviewPanelProps } from "@/lib/moss-tabs/src";

import { WorkspaceDangerZoneSection } from "./WorkspaceDangerZoneSection";
import { WorkspaceDataSection } from "./WorkspaceDataSection";
import { WorkspaceNameSection } from "./WorkspaceNameSection";
import { WorkspaceStartupSection } from "./WorkspaceStartupSection";

export const WorkspaceSettings = ({ ...props }: IDockviewPanelProps) => {
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

  useEffect(() => {
    if (activeWorkspace) {
      setName(activeWorkspace.name);
    }
  }, [activeWorkspace]);

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
      <div className="flex h-full items-center justify-center text-(--moss-primary-text)">
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
          description="This will delete the monitors, scheduled runs and integrations and deactivate the mock servers associated with collections in the workspace."
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
