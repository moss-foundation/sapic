import { useState, useEffect } from "react";

import { InputOutlined } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { useUpdateWorkspace } from "@/hooks/workbench/useUpdateWorkspace";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";

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
    <div className="space-y-6">
      {/* Name Section */}
      <div className="mt-4">
        <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Name:</h3>
        <div className="w-[400px]">
          <InputOutlined
            size="md"
            value={name}
            onChange={(e) => setName(e.target.value)}
            onBlur={handleBlur}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                handleSave();
              } else if (e.key === "Escape") {
                e.preventDefault();
                handleReset();
              }
            }}
            placeholder="Enter workspace name..."
          />
        </div>
        <p className="mt-1 text-xs text-gray-500">
          Invalid filename characters (e.g. / \ : * ? " &lt; &gt; |) will be escaped
        </p>
        {hasChanges && (
          <div className="mt-3 flex items-center gap-2">
            <ButtonPrimary onClick={handleSave} disabled={isPending || !name.trim()} size="md">
              {isPending ? "Saving..." : "Save"}
            </ButtonPrimary>
            <ButtonNeutralOutlined onClick={handleReset} size="md">
              Cancel
            </ButtonNeutralOutlined>
          </div>
        )}
      </div>

      {/* Startup Section */}
      <div className="mt-6">
        <h3 className="mb-4 font-medium text-[var(--moss-select-text-outlined)]">Startup</h3>
        <div className="space-y-3">
          <label className="flex items-center gap-3">
            <input
              type="checkbox"
              checked={reopenOnNextSession}
              onChange={(e) => setReopenOnNextSession(e.target.checked)}
              className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-[var(--moss-select-text-outlined)]">
              Reopen this workspace on next session
            </span>
          </label>
          <label className="flex items-center gap-3">
            <input
              type="checkbox"
              checked={openPreviousWindows}
              onChange={(e) => setOpenPreviousWindows(e.target.checked)}
              className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-[var(--moss-select-text-outlined)]">Open previous windows and tabs</span>
          </label>
        </div>
      </div>

      {/* Data and Storage Section */}
      <div className="mt-6">
        <h3 className="mb-4 font-medium text-[var(--moss-select-text-outlined)]">Data and Storage</h3>
        <div className="space-y-3">
          <div>
            <p className="text-sm font-medium text-[var(--moss-select-text-outlined)]">Delete this workspace</p>
            <p className="text-xs text-gray-500">Last checked: 25 MB</p>
          </div>
          <div>
            <ButtonNeutralOutlined size="md" disabled>
              Clear
            </ButtonNeutralOutlined>
          </div>
        </div>
      </div>

      {/* Danger Zone Section */}
      <div className="mt-6">
        <h3 className="mb-4 font-medium text-red-600">Danger Zone</h3>
        <div className="space-y-3">
          <div>
            <p className="text-sm font-medium text-[var(--moss-select-text-outlined)]">Delete this workspace</p>
            <p className="text-xs text-gray-500">
              Once you delete a workspace, there is no going back. Please be certain.
            </p>
          </div>
          <div>
            <ButtonPrimary size="md" disabled className="bg-red-600 text-white hover:bg-red-700 disabled:bg-red-400">
              Delete
            </ButtonPrimary>
          </div>
        </div>
      </div>
    </div>
  );
};
