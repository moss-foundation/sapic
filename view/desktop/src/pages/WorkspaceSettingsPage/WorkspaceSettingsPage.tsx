import { useEffect, useState } from "react";

import { ConfirmationModal } from "@/components/Modals/ConfirmationModal";
import { useDeleteWorkspace } from "@/hooks/workbench/useDeleteWorkspace";
import { useUpdateWorkspace } from "@/hooks/workbench/useUpdateWorkspace";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";

import { WorkspaceDangerZoneSection } from "./WorkspaceDangerZoneSection";
import { WorkspaceDataSection } from "./WorkspaceDataSection";
import { WorkspaceNameSection } from "./WorkspaceNameSection";
import { WorkspaceStartupSection } from "./WorkspaceStartupSection";

export const WorkspaceSettings = () => {
  const workspace = useActiveWorkspace();
  const { mutate: updateWorkspace, isPending } = useUpdateWorkspace();
  const { mutate: deleteWorkspace } = useDeleteWorkspace();

  const [name, setName] = useState(workspace?.displayName || "");
  const [reopenOnNextSession, setReopenOnNextSession] = useState(false);
  const [openPreviousWindows, setOpenPreviousWindows] = useState(false);
  const [showDeleteConfirmModal, setShowDeleteConfirmModal] = useState(false);

  useEffect(() => {
    if (workspace) {
      setName(workspace.displayName);
    }
  }, [workspace]);

  const handleSave = () => {
    if (name.trim() && name.trim() !== workspace?.displayName) {
      updateWorkspace(
        { name: name.trim() },
        {
          onError: (error) => {
            console.error("Failed to update workspace:", error.message);
          },
        }
      );
    }
  };

  // Auto-save when input loses focus
  const handleBlur = () => {
    handleSave();
  };

  // Delete workspace handlers
  const handleDeleteClick = () => {
    setShowDeleteConfirmModal(true);
  };

  const handleDeleteWorkspace = () => {
    if (workspace) {
      deleteWorkspace({ id: workspace.id });
      setShowDeleteConfirmModal(false);
    }
  };

  const closeDeleteConfirmModal = () => {
    setShowDeleteConfirmModal(false);
  };

  if (!workspace) {
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
    <>
      {showDeleteConfirmModal && (
        <ConfirmationModal
          showModal={showDeleteConfirmModal}
          closeModal={closeDeleteConfirmModal}
          title="Delete"
          message={`Delete "${workspace?.displayName}"?`}
          description="This will delete the monitors, scheduled runs and integrations and deactivate the mock servers associated with collections in the workspace."
          confirmLabel="Delete"
          cancelLabel="Close"
          onConfirm={handleDeleteWorkspace}
          variant="danger"
        />
      )}

      <div className="flex h-full justify-center">
        <div className="w-full max-w-2xl space-y-6 px-6 py-5">
          <WorkspaceNameSection name={name} setName={setName} onSave={handleSave} onBlur={handleBlur} />

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
    </>
  );
};
