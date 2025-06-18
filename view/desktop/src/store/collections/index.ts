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

  startCollectionsStream: () => void;
  streamedCollections: StreamCollectionsEvent[];
  areCollectionsStreaming: boolean;

  streamedCollectionEntries: EntryInfo[];
  areCollectionEntriesStreaming: boolean;
  startCollectionEntriesStream: (collectionId: string) => void;
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

  areCollectionsStreaming: false,
  streamedCollections: [],
  startCollectionsStream: async () => {
    try {
      set({
        areCollectionsStreaming: true,
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

        get().startCollectionEntriesStream(collection.id);
      };

      await invokeTauriIpc("stream_collections", {
        channel: onCollectionEvent,
      });
    } catch (error) {
      console.error("Failed to set up stream_collections:", error);
    } finally {
      set({ areCollectionsStreaming: false });
    }
  },

  areCollectionEntriesStreaming: false,
  streamedCollectionEntries: [],
  startCollectionEntriesStream: async (collectionId: string) => {
    try {
      set({ areCollectionEntriesStreaming: true });

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
    } finally {
      set({ areCollectionEntriesStreaming: false });
    }
  },
}));
