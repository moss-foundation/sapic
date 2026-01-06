import { useState } from "react";

import { useFetchResourcesForPath, useUpdateProjectResource } from "@/adapters";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { StreamResourcesEvent } from "@repo/moss-project";
import { join } from "@tauri-apps/api/path";

export const useRenameResourceForm = (resource: StreamResourcesEvent, projectId: string) => {
  const { fetchResourcesForPath } = useFetchResourcesForPath();
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
        await fetchResourcesForPath(projectId, newPath);
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
