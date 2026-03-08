import { useContext, useState } from "react";

import { resourceService } from "@/domains/resource/resourceService";
import { join } from "@tauri-apps/api/path";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ResourceNode } from "../../types";

export const useResourceNodeRenamingForm = (node: ResourceNode) => {
  const { id } = useContext(ProjectTreeContext);

  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const handleRenamingFormSubmit = async (newName: string) => {
    const trimmedNewName = newName.trim();

    try {
      if (trimmedNewName === node.name) {
        return;
      }

      if (node.kind === "Dir") {
        await resourceService.update(id, {
          DIR: {
            id: node.id,
            name: trimmedNewName,
          },
        });

        const newPath = await join(...node.path.segments.slice(0, node.path.segments.length - 1), trimmedNewName);
        await resourceService.list({ projectId: id, mode: { "RELOAD_PATH": newPath } });
      } else {
        await resourceService.update(id, {
          ITEM: {
            id: node.id,
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
