import { create } from "zustand";

import { CollectionTree } from "@/components/CollectionTree/types";
import {
  checkIfTreeIsCollapsed,
  checkIfTreeIsExpanded,
  collapseAllNodes,
  expandAllNodes,
} from "@/components/CollectionTree/utils";

import AzureDevOpsTestCollection from "../../../assets/AzureDevOpsTestCollection.json";
import SapicTestCollection from "../../../assets/SapicTestCollection.json";
import WhatsAppBusinessTestCollection from "../../../assets/WhatsAppBusinessTestCollection.json";

export interface CollectionsPlaceholderSliceState {
  collections: CollectionTree[];
  setCollections: (collections: CollectionTree[]) => void;
  expandAll: () => void;
  collapseAll: () => void;
  updateCollection: (collection: CollectionTree) => void;
}

export const useCollectionsPlaceholderSlice = create<CollectionsPlaceholderSliceState>((set, get) => ({
  collections: [
    {
      ...SapicTestCollection,
      name: "Sapic Test Collection",
    } as unknown as CollectionTree,
    {
      ...AzureDevOpsTestCollection,
      name: "Azure DevOps Test Collection",
    } as unknown as CollectionTree,
    {
      ...WhatsAppBusinessTestCollection,
      name: "WhatsApp Business Test Collection",
    } as unknown as CollectionTree,
  ],
  setCollections: (collections: CollectionTree[]) => {
    set({ collections });
  },
  expandAll: () => {
    const allFoldersAreExpanded = get().collections.every((collection) => checkIfTreeIsExpanded(collection.tree));

    if (allFoldersAreExpanded) return;

    set((state) => ({
      collections: state.collections.map((collection) => {
        return {
          ...collection,
          tree: expandAllNodes(collection.tree),
        };
      }),
    }));
  },
  collapseAll: () => {
    const allFoldersAreCollapsed = get().collections.every((collection) => checkIfTreeIsCollapsed(collection.tree));

    if (allFoldersAreCollapsed) return;

    set((state) => ({
      collections: state.collections.map((collection) => {
        return {
          ...collection,
          tree: collapseAllNodes(collection.tree),
        };
      }),
    }));
  },
  updateCollection: (updatedCollection: CollectionTree) => {
    set((state) => ({
      collections: state.collections.map((c) => (c.id === updatedCollection.id ? { ...updatedCollection } : c)),
    }));
  },
}));
