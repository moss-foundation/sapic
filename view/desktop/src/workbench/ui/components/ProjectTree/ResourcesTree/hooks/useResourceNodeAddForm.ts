import { useContext, useState } from "react";

import { resourceService } from "@/domains/resource/resourceService";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { createResourceKind } from "../dnd/handlerOperations/createResourceKind.ts";
import { ResourceNode } from "../types";

export const useResourceNodeAddForm = (parentNode: ResourceNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { currentWorkspaceId } = useCurrentWorkspace();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (name: string) => {
    const path = "path" in parentNode ? parentNode.path.raw || "" : "";
    const resourceClass = "class" in parentNode ? parentNode.class : "endpoint";
    const newOrder = parentNode.childNodes.length + 1;

    const newResource = createResourceKind({
      name: name.trim(),
      path,
      isAddingFolder: isAddingFolderNode,
      order: newOrder,
      protocol: resourceClass === "endpoint" ? "Get" : undefined,
      class: resourceClass,
    });

    try {
      setIsAddingFileNode(false);
      setIsAddingFolderNode(false);
      const createdResourceOutput = await resourceService.create(id, newResource);

      await treeItemStateService.putOrder(createdResourceOutput.id, newOrder, currentWorkspaceId);
      await treeItemStateService.putExpanded(createdResourceOutput.id, true, currentWorkspaceId);

      await resourceService.update(id, {
        DIR: {
          id: parentNode.id,
          expanded: true,
        },
      });
    } catch (error) {
      console.error(error);
    }
  };

  const handleAddFormCancel = () => {
    setIsAddingFileNode(false);
    setIsAddingFolderNode(false);
  };

  return {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  };
};
