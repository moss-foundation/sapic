import { useState } from "react";

import { useUpdateProjectResource } from "@/adapters";
import { resourceService } from "@/domains/resource/resourceService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { join } from "@tauri-apps/api/path";

import { ResourceNode } from "../ui/components/ProjectTree/types";

export const useRenameResourceForm = (resource: ResourceNode, projectId: string) => {
  const { mutateAsync: updateProjectResource } = useUpdateProjectResource();
  const { api } = useTabbedPaneStore();

  const [isRenamingResource, setIsRenamingResource] = useState(false);

  const handleRenamingResourceSubmit = async (newName: string) => {
    try {
      const trimmedNewName = newName.trim();

      if (trimmedNewName === resource.name) {
        return;
      }

      if (resource.kind === "Dir") {
        await updateProjectResource({
          projectId,
          updateResourceInput: {
            DIR: {
              id: resource.id,
              name: trimmedNewName,
            },
          },
        });

        const newPath = await join(
          ...resource.path.segments.slice(0, resource.path.segments.length - 1),
          trimmedNewName
        );
        await resourceService.list({ projectId, mode: { "RELOAD_PATH": newPath } });
      } else {
        await updateProjectResource({
          projectId,
          updateResourceInput: {
            ITEM: {
              id: resource.id,
              name: trimmedNewName,
              queryParamsToAdd: [],
              queryParamsToUpdate: [],
              queryParamsToRemove: [],
              pathParamsToAdd: [],
              pathParamsToUpdate: [],
              pathParamsToRemove: [],
              headersToAdd: [],
              headersToUpdate: [],
              headersToRemove: [],
            },
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
