import { create } from "zustand";

import { Collection } from "@/components/CollectionTree/types";
import {
  checkIfTreeIsCollapsed,
  checkIfTreeIsExpanded,
  collapseAllNodes,
  expandAllNodes,
} from "@/components/CollectionTree/utils";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { EntryInfo } from "@repo/moss-collection";
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

  streamedCollectionEntries: EntryInfo[];
  isBeingStreamedCollectionEntries: boolean;
  getCollectionEntries: (collectionId: string) => void;
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
    try {
      set({
        isBeingStreamed: true,
        streamedCollections: [],
        streamedCollectionEntries: [],
      });

      const onCollectionEvent = new Channel<StreamCollectionsEvent>();

      onCollectionEvent.onmessage = (collection) => {
        set((state) => {
          const existingCollection = state.streamedCollections.find((c) => c.id === collection.id);
          if (existingCollection) {
            return state;
          }
          return { ...state, streamedCollections: [...state.streamedCollections, collection] };
        });

        get().getCollectionEntries(collection.id);
      };

      await invokeTauriIpc("stream_collections", {
        channel: onCollectionEvent,
      });
    } catch (error) {
      console.error("Failed to set up stream_collections:", error);
    } finally {
      set({ isBeingStreamed: false });
    }
  },
  streamedCollections: [],
  isBeingStreamed: false,

  streamedCollectionEntries: [],
  isBeingStreamedCollectionEntries: false,
  getCollectionEntries: async (collectionId: string) => {
    try {
      const onCollectionEntryEvent = new Channel<EntryInfo>();

      onCollectionEntryEvent.onmessage = (collectionEntry) => {
        set((state) => {
          return { ...state, streamedCollectionEntries: [...state.streamedCollectionEntries, collectionEntry] };
        });
      };

      await invokeTauriIpc("stream_collection_entries", {
        collectionId,
        channel: onCollectionEntryEvent,
      });
    } catch (error) {
      console.error("Failed to get collection entries:", error);
    }
  },
}));
