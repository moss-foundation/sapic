import { useContext, useState } from "react";

import { resourceService } from "@/domains/resource/resourceService";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTree } from "../../types";
import { createResourceKind } from "../../utils";

export const useRootNodeAddForm = (node: ProjectTree) => {
  const { id } = useContext(ProjectTreeContext);
  const { currentWorkspaceId } = useCurrentWorkspace();

  const [isAddingRootFileNode, setIsAddingRootFileNode] = useState(false);
  const [isAddingRootFolderNode, setIsAddingRootFolderNode] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newName = name.trim();
    const newOrder = node.resourcesTree.childNodes.length + 1;
    const newResource = createResourceKind({
      name: newName,
      path: "",
      class: "endpoint",
      isAddingFolder: isAddingRootFolderNode,
      order: newOrder,
      protocol: "Get",
    });

    try {
      setIsAddingRootFileNode(false);
      setIsAddingRootFolderNode(false);

      const createdResourceOutput = await resourceService.create(id, newResource);

      await treeItemStateService.putOrder(createdResourceOutput.id, newOrder, currentWorkspaceId);
      await treeItemStateService.putExpanded(createdResourceOutput.id, true, currentWorkspaceId);
    } catch (error) {
      console.error(error);
    } finally {
      setIsAddingRootFileNode(false);
      setIsAddingRootFolderNode(false);
    }
  };

  const handleRootAddFormCancel = () => {
    setIsAddingRootFileNode(false);
    setIsAddingRootFolderNode(false);
  };

  return {
    isAddingRootFileNode,
    isAddingRootFolderNode,
    setIsAddingRootFileNode,
    setIsAddingRootFolderNode,
    handleRootAddFormSubmit,
    handleRootAddFormCancel,
  };
};
