import { useState } from "react";

import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { WorkspaceInfo } from "@repo/window";

import { useUpdateWorkspace } from "./workbench/useUpdateWorkspace";

export const useRenameWorkspace = (workspace: WorkspaceInfo | undefined) => {
  const { mutateAsync: updateWorkspace } = useUpdateWorkspace();

  const { api } = useTabbedPaneStore();

  const [isRenamingWorkspace, setIsRenamingWorkspace] = useState(false);

  const handleRenamingWorkspaceSubmit = async (newName: string) => {
    if (!workspace) {
      return;
    }

    try {
      const trimmedNewName = newName.trim();

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
