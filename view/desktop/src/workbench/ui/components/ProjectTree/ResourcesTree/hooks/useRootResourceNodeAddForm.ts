import { useContext, useState } from "react";

import { resourceService } from "@/domains/resource/resourceService";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ResourcesTreeRoot } from "../../TreeRoot/types";
import { createResourceKind } from "../dnd/handlerOperations/createResourceKind.ts";

interface UseRootResourceNodeAddFormProps {
  tree: ResourcesTreeRoot;
}

export const useRootResourceNodeAddForm = ({ tree }: UseRootResourceNodeAddFormProps) => {
  const { id } = useContext(ProjectTreeContext);

  const { currentWorkspaceId } = useCurrentWorkspace();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (name: string) => {
    const newOrder = tree.childNodes.length + 1;

    const newResource = createResourceKind({
      name: name.trim(),
      path: "",
      isAddingFolder: isAddingFolderNode,
      order: newOrder,
      protocol: "Get",
      class: "endpoint",
    });

    try {
      setIsAddingFileNode(false);
      setIsAddingFolderNode(false);

      const createdResourceOutput = await resourceService.create(id, newResource);

      await treeItemStateService.putOrder(createdResourceOutput.id, newOrder, currentWorkspaceId);
      await treeItemStateService.putExpanded(createdResourceOutput.id, true, currentWorkspaceId);
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
