import { useContext, useState } from "react";

import { useCollectionsStore } from "@/store/collections";
import { CreateEntryInput } from "@repo/moss-collection";

import { TreeContext } from "../../Tree";

const createEntry = (name: string, isAddingFolder: boolean): CreateEntryInput => {
  if (isAddingFolder) {
    return {
      dir: {
        name,
        path: "requests",
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
      path: "requests",
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

export const useRootNodeAddForm = () => {
  const { treeId } = useContext(TreeContext);
  const { createCollectionEntry } = useCollectionsStore();

  const [isAddingRootNodeFile, setIsAddingRootNodeFile] = useState(false);
  const [isAddingRootNodeFolder, setIsAddingRootNodeFolder] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newEntry = createEntry(name, isAddingRootNodeFolder);

    await createCollectionEntry({
      collectionId: treeId,
      input: newEntry,
    });

    setIsAddingRootNodeFile(false);
    setIsAddingRootNodeFolder(false);
  };

  const handleRootAddFormCancel = () => {
    setIsAddingRootNodeFile(false);
    setIsAddingRootNodeFolder(false);
  };

  return {
    isAddingRootNodeFile,
    isAddingRootNodeFolder,
    setIsAddingRootNodeFile,
    setIsAddingRootNodeFolder,
    handleRootAddFormSubmit,
    handleRootAddFormCancel,
  };
};
