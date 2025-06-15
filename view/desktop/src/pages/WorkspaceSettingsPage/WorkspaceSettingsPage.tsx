import { useState, useEffect } from "react";

import { useUpdateWorkspace } from "@/hooks/workbench/useUpdateWorkspace";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";

import { WorkspaceNameSection } from "./WorkspaceNameSection";
import { WorkspaceStartupSection } from "./WorkspaceStartupSection";
import { WorkspaceDataSection } from "./WorkspaceDataSection";
import { WorkspaceDangerZoneSection } from "./WorkspaceDangerZoneSection";

export const WorkspaceSettings = () => {
  const workspace = useActiveWorkspace();
  const { mutate: updateWorkspace, isPending } = useUpdateWorkspace();

  const [name, setName] = useState(workspace?.displayName || "");
  const [hasChanges, setHasChanges] = useState(false);
  const [reopenOnNextSession, setReopenOnNextSession] = useState(false);
  const [openPreviousWindows, setOpenPreviousWindows] = useState(false);

  // Update local state when workspace changes
  useEffect(() => {
    if (workspace) {
      setName(workspace.displayName);
      setHasChanges(false);
    }
  }, [workspace]);

  // Track changes
  useEffect(() => {
    setHasChanges(name !== (workspace?.displayName || ""));
  }, [name, workspace?.displayName]);

  const handleSave = () => {
    if (name.trim() && name.trim() !== workspace?.displayName) {
      updateWorkspace(
        { name: name.trim() },
        {
          onSuccess: () => {
            setHasChanges(false);
          },
          onError: (error) => {
            console.error("Failed to update workspace:", error.message);
          },
        }
      );
    }
  };

  const handleReset = () => {
    if (workspace) {
      setName(workspace.displayName);
      setHasChanges(false);
    }
  };

  // Auto-save when input loses focus
  const handleBlur = () => {
    if (hasChanges) {
      handleSave();
    }
  };

  if (!workspace) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <h2 className="text-lg font-semibold text-[var(--moss-select-text-outlined)]">No Active Workspace</h2>
          <p className="text-sm text-[var(--moss-select-text-outlined)]">
            Please select a workspace to view its settings.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full justify-center">
      <div className="w-full max-w-2xl space-y-6 px-6 py-6">
        <WorkspaceNameSection
          name={name}
          setName={setName}
          hasChanges={hasChanges}
          isPending={isPending}
          onSave={handleSave}
          onReset={handleReset}
          onBlur={handleBlur}
        />

        <WorkspaceStartupSection
          reopenOnNextSession={reopenOnNextSession}
          setReopenOnNextSession={setReopenOnNextSession}
          openPreviousWindows={openPreviousWindows}
          setOpenPreviousWindows={setOpenPreviousWindows}
        />

        <WorkspaceDataSection />

        <WorkspaceDangerZoneSection />
      </div>
    </div>
  );
};
