import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { CreateEntryInput, DirConfigurationModel, ItemConfigurationModel } from "@repo/moss-collection";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

//FIXME: This is a temporary solution until we have a proper configuration model
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

//FIXME: This is a temporary solution until we have a proper configuration model
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

//FIXME: This is a temporary solution until we have a proper configuration model
const createEntry = (parentNode: TreeCollectionNode, name: string, isAddingFolder: boolean): CreateEntryInput => {
  const baseEntry = {
    name,
    path: parentNode.path.raw,
  };

  if (isAddingFolder) {
    return {
      dir: {
        ...baseEntry,
        order: parentNode.childNodes.length + 1,
        configuration: createDirConfiguration(parentNode.class),
      },
    };
  }

  return {
    item: {
      ...baseEntry,
      order: parentNode.childNodes.length + 1,
      configuration: createItemConfiguration(parentNode.class),
    },
  };
};

export const useNodeAddForm = (parentNode: TreeCollectionNode, onNodeUpdate: (node: TreeCollectionNode) => void) => {
  const { id } = useContext(TreeContext);
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (name: string) => {
    const newEntry = createEntry(parentNode, name, isAddingFolderNode);

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
