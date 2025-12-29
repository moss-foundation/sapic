import { useState } from "react";

import { useDeleteWorkspace } from "@/adapters/tanstackQuery/workspace/useDeleteWorkspace";
import { useModal } from "@/hooks";
import { useRenameWorkspace } from "@/hooks/useRenameWorkspace";
import { useCurrentWorkspace } from "@/hooks/workspace/derived/useCurrentWorkspace";
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
  const { currentWorkspace } = useCurrentWorkspace();

  const { mutate: deleteWorkspace, isPending: isDeleting } = useDeleteWorkspace();

  const [name, setName] = useState(currentWorkspace?.name || "");
  const [reopenOnNextSession, setReopenOnNextSession] = useState(false);
  const [openPreviousWindows, setOpenPreviousWindows] = useState(false);

  const {
    openModal: openDeleteWorkspaceModal,
    closeModal: closeDeleteWorkspaceModal,
    showModal: isDeleteWorkspaceModalOpen,
  } = useModal();

  const { isRenamingWorkspace, setIsRenamingWorkspace, handleRenamingWorkspaceSubmit, handleRenamingWorkspaceCancel } =
    useRenameWorkspace(currentWorkspace);

  const handleBlur = () => {
    handleRenamingWorkspaceSubmit(name);
  };

  const handleDeleteClick = () => {
    openDeleteWorkspaceModal();
  };

  const handleDeleteWorkspace = () => {
    if (currentWorkspace) {
      deleteWorkspace(
        { id: currentWorkspace.id },
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

  return (
    <PageView>
      <PageHeader
        icon="Workspace"
        title={currentWorkspace?.name}
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
          message={`Delete "${currentWorkspace?.name}"?`}
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
