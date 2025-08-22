import { useState } from "react";

import { WorkspaceInfo } from "@repo/moss-app";

import { useUpdateWorkspace } from "./workbench/useUpdateWorkspace";

export const useRenameWorkspace = (workspace: WorkspaceInfo | null) => {
  const { mutateAsync: updateWorkspace } = useUpdateWorkspace();

  const [isRenamingWorkspace, setIsRenamingWorkspace] = useState(false);

  const handleRenamingWorkspaceSubmit = async (newName: string) => {
    if (!workspace) {
      return;
    }

    try {
      if (newName.trim() === workspace.name) {
        return;
      }

      await updateWorkspace({
        name: newName.trim(),
      });
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingWorkspace(false);
    }
  };

  const handleRenamingWorkspaceCancel = () => {
    setIsRenamingWorkspace(false);
  };

  return {
    isRenamingWorkspace,
    setIsRenamingWorkspace,
    handleRenamingWorkspaceSubmit,
    handleRenamingWorkspaceCancel,
  };
};
