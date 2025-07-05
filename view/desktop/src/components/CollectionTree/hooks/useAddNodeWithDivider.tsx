import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { CreateEntryInput, ItemConfigurationModel } from "@repo/moss-collection";

import { TreeContext } from "../..";
import { TreeCollectionNode } from "../types";
import { updateNodesOrder } from "../utils";

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
const createEntry = async (parentNode: TreeCollectionNode, name: string, order: number): Promise<CreateEntryInput> => {
  return {
    item: {
      name,
      path: parentNode.path.raw,
      order,
      configuration: createItemConfiguration(parentNode.class),
    },
  };
};

export const useAddNodeWithDivider = (
  node: TreeCollectionNode,
  parentNode: TreeCollectionNode,
  position: "before" | "after"
) => {
  const { id: collectionId } = useContext(TreeContext);
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();
  const [isAddingDividerNode, setIsAddingDividerNode] = useState(false);

  const handleAddDividerFormSubmit = async (name: string) => {
    try {
      if (!parentNode) {
        console.error("Parent node is undefined");
        return;
      }

      if (!parentNode.childNodes) {
        console.warn("Parent node childNodes is undefined, initializing as empty array");
        parentNode.childNodes = [];
      }

      const sortedParentNodeChildNodes = [...parentNode.childNodes].sort((a, b) => a.order! - b.order!);

      const currentNodeIndex = sortedParentNodeChildNodes.findIndex((child) => child.id === node.id);

      if (currentNodeIndex === -1) {
        console.error("Current node not found in parent's children");
        console.log(
          "Available child IDs:",
          sortedParentNodeChildNodes.map((child) => child.id)
        );
        return;
      }

      const insertionIndex = position === "before" ? currentNodeIndex : currentNodeIndex + 1;
      const newEntryOrder = insertionIndex + 1;

      const newEntry = await createEntry(parentNode, name, newEntryOrder);

      const result = await createCollectionEntry({
        collectionId,
        input: newEntry,
      });

      if (!result) {
        console.error("Failed to create collection entry");
        return;
      }

      const newChildNodes = [
        ...sortedParentNodeChildNodes.slice(0, insertionIndex),
        {
          ...result,
          childNodes: [],
        },
        ...sortedParentNodeChildNodes.slice(insertionIndex),
      ];

      const updatedChildNodes = updateNodesOrder(newChildNodes);

      const nodesToUpdate = updatedChildNodes.slice(insertionIndex + 1);

      if (nodesToUpdate.length > 0) {
        await batchUpdateCollectionEntry({
          collectionId,
          entries: {
            entries: nodesToUpdate.map((nodeToUpdate) =>
              nodeToUpdate.kind === "Dir"
                ? {
                    "DIR": {
                      id: nodeToUpdate.id,
                      path: nodeToUpdate.path.raw,
                      order: nodeToUpdate.order,
                    },
                  }
                : {
                    "ITEM": {
                      id: nodeToUpdate.id,
                      path: nodeToUpdate.path.raw,
                      order: nodeToUpdate.order,
                    },
                  }
            ),
          },
        });
      }
    } catch (error) {
      console.error("Failed to add divider node:", error);
    } finally {
      setIsAddingDividerNode(false);
    }
  };

  const handleAddDividerFormCancel = () => {
    setIsAddingDividerNode(false);
  };

  return {
    isAddingDividerNode,
    setIsAddingDividerNode,
    handleAddDividerFormSubmit,
    handleAddDividerFormCancel,
  };
};
