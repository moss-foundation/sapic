import { useContext, useState } from "react";

import { useCreateProjectResource, useUpdateProjectResource } from "@/adapters";
import { useCurrentWorkspace } from "@/hooks";
import { usePutTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useUpdateTreeItemState";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../../types";
import { createResourceKind } from "../../utils";

export const useNodeAddForm = (parentNode: ProjectTreeNode | ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutateAsync: createProjectResource } = useCreateProjectResource();
  const { mutateAsync: updateProjectResource } = useUpdateProjectResource();

  const { mutateAsync: updateTreeItemState } = usePutTreeItemState();

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

      const createdResourceOutput = await createProjectResource({
        projectId: id,
        input: newResource,
      });

      await updateTreeItemState({
        treeItemState: {
          id: createdResourceOutput.id,
          order: newOrder,
          expanded: true,
        },
        workspaceId: currentWorkspaceId,
      });

      await updateProjectResource({
        projectId: id,
        updateResourceInput: {
          DIR: {
            id: parentNode.id,
            expanded: true,
          },
        },
      });
    } catch (error) {
      console.error(error);
    } finally {
      setIsAddingFileNode(false);
      setIsAddingFolderNode(false);
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
