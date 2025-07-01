import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";
import { CreateEntryInput, DirConfigurationModel, ItemConfigurationModel } from "@repo/moss-collection";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

const createDirConfiguration = (nodeClass: TreeCollectionNode["class"]): DirConfigurationModel => {
  switch (nodeClass) {
    case "Request":
      return { request: { http: {} } };
    case "Endpoint":
      return { request: { http: {} } };
    case "Component":
      return { component: {} };
    case "Schema":
      return { schema: {} };
    default:
      return { request: { http: {} } };
  }
};

const createItemConfiguration = (nodeClass: TreeCollectionNode["class"]): ItemConfigurationModel => {
  switch (nodeClass) {
    case "Request":
      return {
        request: {
          http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
    case "Endpoint":
      return {
        endpoint: {
          Http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
    case "Component":
      return { component: {} };
    case "Schema":
      return { schema: {} };
    default:
      return {
        request: {
          http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
  }
};

const createEntry = (parentNode: TreeCollectionNode, name: string, isAddingFolder: boolean): CreateEntryInput => {
  const baseEntry = {
    name,
    path: parentNode.path.raw,
  };

  if (isAddingFolder) {
    return {
      dir: {
        ...baseEntry,
        order: 0,
        configuration: createDirConfiguration(parentNode.class),
      },
    };
  }

  return {
    item: {
      ...baseEntry,
      order: 0,
      configuration: createItemConfiguration(parentNode.class),
    },
  };
};

export const useNodeAddForm = (parentNode: TreeCollectionNode, onNodeUpdate: (node: TreeCollectionNode) => void) => {
  const { id } = useContext(TreeContext);
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (name: string) => {
    const newEntry = createEntry(parentNode, name, isAddingFolderNode);

    const result = await createCollectionEntry({
      collectionId: id,
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
