import { useContext, useState } from "react";

import { useCreateProjectEntry } from "@/hooks";
import { useUpdateProjectEntry } from "@/hooks/project/useUpdateProjectEntry";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../../types";
import { createEntryKind } from "../../utils";

export const useNodeAddForm = (parentNode: ProjectTreeNode | ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { mutateAsync: createProjectEntry } = useCreateProjectEntry();
  const { mutateAsync: updateProjectEntry } = useUpdateProjectEntry();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (name: string) => {
    const path = "path" in parentNode ? parentNode.path.raw || "" : "";
    const entryClass = "class" in parentNode ? parentNode.class : "Endpoint";

    const newEntry = createEntryKind({
      name: name.trim(),
      path,
      isAddingFolder: isAddingFolderNode,
      order: parentNode.childNodes.length + 1,
      protocol: entryClass === "Endpoint" ? "Get" : undefined,
      class: entryClass,
    });

    try {
      await createProjectEntry({
        projectId: id,
        input: newEntry,
      });

      await updateProjectEntry({
        projectId: id,
        updatedEntry: {
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
