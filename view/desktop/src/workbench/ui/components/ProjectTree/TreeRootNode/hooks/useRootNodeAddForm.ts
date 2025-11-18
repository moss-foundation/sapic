import { useContext, useState } from "react";

import { useCreateProjectResource } from "@/workbench/adapters/tanstackQuery/project";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeRootNode } from "../../types";
import { createResourceKind } from "../../utils";

export const useRootNodeAddForm = (node: ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { mutateAsync: createProjectResource } = useCreateProjectResource();

  const [isAddingRootFileNode, setIsAddingRootFileNode] = useState(false);
  const [isAddingRootFolderNode, setIsAddingRootFolderNode] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newName = name.trim();
    const newResource = createResourceKind({
      name: newName,
      path: "",
      class: "endpoint",
      isAddingFolder: isAddingRootFolderNode,
      order: node.childNodes.length + 1,
      protocol: "Get",
    });

    try {
      setIsAddingRootFileNode(false);
      setIsAddingRootFolderNode(false);

      await createProjectResource({
        projectId: id,
        input: newResource,
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
