import { useContext, useState } from "react";

import { useCreateProjectResource } from "@/adapters";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/domains/treeItemState/service";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeRootNode } from "../../types";
import { createResourceKind } from "../../utils";

export const useRootNodeAddForm = (node: ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutateAsync: createProjectResource } = useCreateProjectResource();

  const [isAddingRootFileNode, setIsAddingRootFileNode] = useState(false);
  const [isAddingRootFolderNode, setIsAddingRootFolderNode] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newName = name.trim();
    const newOrder = node.childNodes.length + 1;
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

      const createdResourceOutput = await createProjectResource({
        projectId: id,
        input: newResource,
      });

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
