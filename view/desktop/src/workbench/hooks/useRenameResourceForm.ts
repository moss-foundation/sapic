import { useState } from "react";

import { resourceService } from "@/domains/resource/resourceService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { join } from "@tauri-apps/api/path";

import { ResourceNode } from "../ui/components/ProjectTree/types";

export const useRenameResourceForm = (resource: ResourceNode, projectId: string) => {
  const { api } = useTabbedPaneStore();

  const [isRenamingResource, setIsRenamingResource] = useState(false);

  const handleRenamingResourceSubmit = async (newName: string) => {
    try {
      const trimmedNewName = newName.trim();

      if (trimmedNewName === resource.name) {
        return;
      }

      if (resource.kind === "Dir") {
        await resourceService.update(projectId, {
          DIR: {
            id: resource.id,
            name: trimmedNewName,
          },
        });

        const newPath = await join(
          ...resource.path.segments.slice(0, resource.path.segments.length - 1),
          trimmedNewName
        );
        await resourceService.list({ projectId, mode: { "RELOAD_PATH": newPath } });
      } else {
        await resourceService.update(projectId, {
          ITEM: {
            id: resource.id,
            name: trimmedNewName,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
          },
        });
      }
      const panel = api?.getPanel(resource.id);
      if (panel) {
        panel.setTitle(trimmedNewName);
      }
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingResource(false);
    }
  };

  const handleRenamingResourceCancel = () => {
    setIsRenamingResource(false);
  };

  return {
    isRenamingResource,
    setIsRenamingResource,
    handleRenamingResourceSubmit,
    handleRenamingResourceCancel,
  };
};
