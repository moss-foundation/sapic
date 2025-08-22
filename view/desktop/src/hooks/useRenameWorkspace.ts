import { useState } from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";
import { WorkspaceInfo } from "@repo/moss-app";

import { useUpdateWorkspace } from "./workbench/useUpdateWorkspace";

export const useRenameWorkspace = (workspace: WorkspaceInfo | null) => {
  const { mutateAsync: updateWorkspace } = useUpdateWorkspace();

  const [isRenamingWorkspace, setIsRenamingWorkspace] = useState(false);

  const { api } = useTabbedPaneStore();

  const handleRenamingWorkspaceSubmit = async (newName: string) => {
    if (!workspace) {
      return;
    }

    const trimmedNewName = newName.trim();

    try {
      if (trimmedNewName === workspace.name) {
        return;
      }

      await updateWorkspace({
        name: trimmedNewName,
      });

      const panel = api?.getPanel("WorkspaceSettings");
      if (panel) {
        panel.setTitle(trimmedNewName);
      }
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
