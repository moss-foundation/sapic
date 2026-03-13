import { useState } from "react";

import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { resourceService } from "@/domains/resource/resourceService";
import { createTransaction } from "@tanstack/db";
import { join } from "@tauri-apps/api/path";

import { ResourceNode } from "../types";

interface UseResourceNodeRenamingFormProps {
  node: ResourceNode;
  projectId: string;
}

export const useResourceNodeRenamingForm = ({ node, projectId }: UseResourceNodeRenamingFormProps) => {
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const handleRenamingFormSubmit = async (newName: string) => {
    const trimmedNewName = newName.trim();
    if (trimmedNewName === node.name) {
      setIsRenamingNode(false);
      return;
    }

    setIsRenamingNode(false);

    const tx = createTransaction({
      autoCommit: false,
      mutationFn: async () => {
        if (node.kind === "Dir") {
          await resourceService.update(projectId, {
            DIR: {
              id: node.id,
              name: trimmedNewName,
            },
          });

          const newPath = await join(...node.path.segments.slice(0, node.path.segments.length - 1), trimmedNewName);
          await resourceService.list({ projectId, mode: { "RELOAD_PATH": newPath } });
        } else {
          await resourceService.update(projectId, {
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
      },
    });

    tx.mutate(() => {
      if (resourceSummariesCollection.has(node.id)) {
        resourceSummariesCollection.update(node.id, (draft) => {
          draft.name = trimmedNewName;
        });
      }
      if (resourceDetailsCollection.has(node.id)) {
        resourceDetailsCollection.update(node.id, (draft) => {
          draft.name = trimmedNewName;
        });
      }
    });

    await tx.commit();
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
