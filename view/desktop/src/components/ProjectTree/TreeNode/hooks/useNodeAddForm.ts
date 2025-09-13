import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../../types";
import { createEntryKind } from "../../utils";

export const useNodeAddForm = (parentNode: ProjectTreeNode | ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();

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
      await createCollectionEntry({
        collectionId: id,
        input: newEntry,
      });

      await updateCollectionEntry({
        collectionId: id,
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
