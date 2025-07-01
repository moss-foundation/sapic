import { useContext, useState } from "react";

import { useCollectionsStore } from "@/store/collections";
import { CreateEntryInput } from "@repo/moss-collection";

import { TreeContext } from "../../Tree";
import { TreeCollectionRootNode } from "../../types";

const createEntry = (name: string, isAddingFolder: boolean): CreateEntryInput => {
  if (isAddingFolder) {
    return {
      dir: {
        name,
        path: "requests",
        order: 0, // FIXME: Temporary hardcoded, to avoid error from the backend
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
      order: 0, // FIXME: Temporary hardcoded, to avoid error from the backend
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

export const useRootNodeAddForm = (
  node: TreeCollectionRootNode,
  onRootNodeUpdate: (node: TreeCollectionRootNode) => void
) => {
  const { treeId } = useContext(TreeContext);
  const { createCollectionEntry } = useCollectionsStore();

  const [isAddingRootNodeFile, setIsAddingRootNodeFile] = useState(false);
  const [isAddingRootNodeFolder, setIsAddingRootNodeFolder] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newEntry = createEntry(name, isAddingRootNodeFolder);

    const result = await createCollectionEntry({
      collectionId: treeId,
      input: newEntry,
    });

    if (result) {
      onRootNodeUpdate({
        ...node,
        requests: {
          ...node.requests,
          childNodes: [
            ...node.requests.childNodes,
            {
              ...result,
              childNodes: [],
            },
          ],
        },
      });
    }

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
