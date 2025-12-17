import { useContext, useState } from "react";

import { useFetchResourcesForPath, useUpdateProjectResource } from "@/adapters";
import { join } from "@tauri-apps/api/path";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeNode } from "../../types";

export const useNodeRenamingForm = (node: ProjectTreeNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { fetchResourcesForPath } = useFetchResourcesForPath();
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const { mutateAsync: updateProjectResource } = useUpdateProjectResource();

  const handleRenamingFormSubmit = async (newName: string) => {
    const trimmedNewName = newName.trim();

    try {
      if (trimmedNewName === node.name) {
        return;
      }

      if (node.kind === "Dir") {
        await updateProjectResource({
          projectId: id,
          updatedResource: {
            DIR: {
              id: node.id,
              name: trimmedNewName,
            },
          },
        });

        const newPath = await join(...node.path.segments.slice(0, node.path.segments.length - 1), trimmedNewName);
        await fetchResourcesForPath(id, newPath);
      } else {
        await updateProjectResource({
          projectId: id,
          updatedResource: {
            ITEM: {
              id: node.id,
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
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingNode(false);
    }
  };

  const handleRenamingFormCancel = () => {
    setIsRenamingNode(false);
  };

  return {
    isRenamingNode,
    setIsRenamingNode,
    handleRenamingFormSubmit,
    handleRenamingFormCancel,
  };
};
