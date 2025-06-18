import { create } from "zustand";

import { Collection } from "@/components/CollectionTree/types";
import {
  checkIfTreeIsCollapsed,
  checkIfTreeIsExpanded,
  collapseAllNodes,
  expandAllNodes,
} from "@/components/CollectionTree/utils";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { Channel } from "@tauri-apps/api/core";
import AzureDevOpsTestCollection from "../../assets/AzureDevOpsTestCollection.json";
import SapicTestCollection from "../../assets/SapicTestCollection.json";
import WhatsAppBusinessTestCollection from "../../assets/WhatsAppBusinessTestCollection.json";

interface CollectionsStoreState {
  collections: Collection[];
  setCollections: (collections: Collection[]) => void;
  expandAll: () => void;
  collapseAll: () => void;
  updateCollection: (collection: Collection) => void;

  refreshCollections: () => void;
  streamedCollections: StreamCollectionsEvent[];
  isBeingStreamed: boolean;
}

export const useCollectionsStore = create<CollectionsStoreState>((set, get) => ({
  collections: [
    SapicTestCollection as Collection,
    AzureDevOpsTestCollection as Collection,
    WhatsAppBusinessTestCollection as Collection,
  ],
  setCollections: (collections: Collection[]) => {
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
  updateCollection: (updatedCollection: Collection) => {
    set((state) => ({
      collections: state.collections.map((c) => (c.id === updatedCollection.id ? { ...updatedCollection } : c)),
    }));
  },
  refreshCollections: async () => {
    console.log("refreshCollections");
    try {
      console.log("refreshCollections try");
      set({ isBeingStreamed: true });
      await new Promise((resolve) => setTimeout(resolve, 1000));

      const onCollectionEvent = new Channel<StreamCollectionsEvent>();
      console.log("refreshCollections onCollectionEvent");
      onCollectionEvent.onmessage = (message) => {
        console.log("Received collection data:", message);

        set((state) => {
          console.log("refreshCollections onCollectionEvent set");
          const existingIndex = state.streamedCollections.findIndex((col) => col.id === message.id);

          if (existingIndex >= 0) {
            const updated = [...state.streamedCollections];
            updated[existingIndex] = message;
            return { ...state, streamedCollections: updated };
          } else {
            return { ...state, streamedCollections: [...state.streamedCollections, message] };
          }
        });

        console.log("refreshCollections onCollectionEvent after set", get().streamedCollections.length);
      };

      console.log("refreshCollections invokeTauriIpc");
      await invokeTauriIpc("stream_collections", {
        channel: onCollectionEvent,
      });
    } catch (error) {
      console.error("Failed to set up stream_collections:", error);
    } finally {
      set({ isBeingStreamed: false });

      console.log("refreshCollections end");
    }
  },
  streamedCollections: [],
  isBeingStreamed: false,
}));
