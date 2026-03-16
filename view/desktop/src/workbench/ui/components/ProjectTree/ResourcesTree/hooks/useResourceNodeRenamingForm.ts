import { useState } from "react";

import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { resourceService } from "@/domains/resource/resourceService";
import { join, sep } from "@tauri-apps/api/path";

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

    const oldName = node.name;
    const dirSegmentIndex = node.path.segments.length - 1;

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

    if (node.kind === "Dir") {
      updateNestedResourcePaths(node.path.segments, dirSegmentIndex, trimmedNewName);
    }

    try {
      if (node.kind === "Dir") {
        await resourceService.update(projectId, {
          DIR: { id: node.id, name: trimmedNewName },
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
    } catch {
      if (resourceSummariesCollection.has(node.id)) {
        resourceSummariesCollection.update(node.id, (draft) => {
          draft.name = oldName;
        });
      }
      if (resourceDetailsCollection.has(node.id)) {
        resourceDetailsCollection.update(node.id, (draft) => {
          draft.name = oldName;
        });
      }

      if (node.kind === "Dir") {
        const renamedSegments = [...node.path.segments];
        renamedSegments[dirSegmentIndex] = trimmedNewName;
        updateNestedResourcePaths(renamedSegments, dirSegmentIndex, oldName);
      }
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

const updateNestedResourcePaths = (oldSegments: string[], segmentIndex: number, newSegmentValue: string) => {
  const platformSeparator = sep();

  resourceSummariesCollection.forEach((resource) => {
    if (resource.path.segments.length <= oldSegments.length) return;

    const isNested = oldSegments.every((seg, i) => resource.path.segments[i] === seg);
    if (!isNested) return;

    resourceSummariesCollection.update(resource.id, (draft) => {
      const newSegments = [...draft.path.segments];
      newSegments[segmentIndex] = newSegmentValue;
      draft.path = { raw: newSegments.join(platformSeparator), segments: newSegments };
    });
  });
};
