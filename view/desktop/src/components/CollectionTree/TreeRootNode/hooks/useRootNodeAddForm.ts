import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";
import { CreateEntryInput } from "@repo/moss-collection";

import { TreeContext } from "../../Tree";
import { TreeCollectionRootNode } from "../../types";

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

export const useRootNodeAddForm = (
  node: TreeCollectionRootNode,
  onRootNodeUpdate: (node: TreeCollectionRootNode) => void
) => {
  const { treeId } = useContext(TreeContext);
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();

  const [isAddingRootNodeFile, setIsAddingRootNodeFile] = useState(false);
  const [isAddingRootNodeFolder, setIsAddingRootNodeFolder] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newEntry = createEntry(name, isAddingRootNodeFolder);

    try {
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
    } catch (error) {
      console.error(error);
    } finally {
      setIsAddingRootNodeFile(false);
      setIsAddingRootNodeFolder(false);
    }
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
