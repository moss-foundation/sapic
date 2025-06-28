import { useContext, useState } from "react";

import { useCollectionsStore } from "@/store/collections";
import { CreateEntryInput } from "@repo/moss-collection";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

const createEntry = (parentNode: TreeCollectionNode, name: string, isAddingFolder: boolean): CreateEntryInput => {
  if (isAddingFolder) {
    return {
      dir: {
        name,
        path: parentNode.path.raw,
        configuration: {
          request: {
            http: {},
          },
        },
      },
    };
  }

  return {
    item: {
      name,
      path: parentNode.path.raw,
      configuration: {
        request: {
          http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      },
    },
  };
};

export const useNodeAddForm = (parentNode: TreeCollectionNode, onNodeUpdate: (node: TreeCollectionNode) => void) => {
  const { treeId } = useContext(TreeContext);
  const { createCollectionEntry } = useCollectionsStore();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (name: string) => {
    const newEntry = createEntry(parentNode, name, isAddingFolderNode);

    const result = await createCollectionEntry({
      collectionId: treeId,
      input: newEntry,
    });

    if (result) {
      onNodeUpdate({
        ...parentNode,
        childNodes: [
          ...parentNode.childNodes,
          {
            ...result,
            childNodes: [],
          },
        ],
      });
    }

    setIsAddingFileNode(false);
    setIsAddingFolderNode(false);
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
