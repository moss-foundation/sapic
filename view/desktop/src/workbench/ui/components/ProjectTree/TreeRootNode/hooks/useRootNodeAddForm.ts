import { useContext, useState } from "react";

import { useCreateProjectResource } from "@/adapters";
import { useCurrentWorkspace } from "@/hooks";
import { usePutTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useUpdateTreeItemState";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeRootNode } from "../../types";
import { createResourceKind } from "../../utils";

export const useRootNodeAddForm = (node: ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutateAsync: updateTreeItemState } = usePutTreeItemState();
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

      await updateTreeItemState({
        treeItemState: { id: createdResourceOutput.id, order: newOrder, expanded: true },
        workspaceId: currentWorkspaceId,
      });
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
