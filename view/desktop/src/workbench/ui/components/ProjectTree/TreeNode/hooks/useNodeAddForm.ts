import { useContext, useState } from "react";

import { useCreateProjectResource } from "@/adapters/tanstackQuery/project";
import { useUpdateProjectResource } from "@/adapters/tanstackQuery/project/useUpdateProjectResource";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../../types";
import { createResourceKind } from "../../utils";

export const useNodeAddForm = (parentNode: ProjectTreeNode | ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { mutateAsync: createProjectResource } = useCreateProjectResource();
  const { mutateAsync: updateProjectResource } = useUpdateProjectResource();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (name: string) => {
    const path = "path" in parentNode ? parentNode.path.raw || "" : "";
    const resourceClass = "class" in parentNode ? parentNode.class : "endpoint";

    const newResource = createResourceKind({
      name: name.trim(),
      path,
      isAddingFolder: isAddingFolderNode,
      order: parentNode.childNodes.length + 1,
      protocol: resourceClass === "endpoint" ? "Get" : undefined,
      class: resourceClass,
    });

    try {
      setIsAddingFileNode(false);
      setIsAddingFolderNode(false);

      await createProjectResource({
        projectId: id,
        input: newResource,
      });
      await updateProjectResource({
        projectId: id,
        updatedResource: {
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
