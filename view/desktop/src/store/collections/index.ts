import { create } from "zustand";

import { Collection } from "@/components/CollectionTree/types";
import {
  checkIfTreeIsCollapsed,
  checkIfTreeIsExpanded,
  collapseAllNodes,
  expandAllNodes,
} from "@/components/CollectionTree/utils";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import {
  CreateEntryInput,
  CreateEntryOutput,
  DeleteEntryInput,
  DeleteEntryOutput,
  EntryInfo,
} from "@repo/moss-collection";
import {
  CreateCollectionInput,
  CreateCollectionOutput,
  DeleteCollectionOutput,
  StreamCollectionsEvent,
} from "@repo/moss-workspace";
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

  isCreateCollectionLoading: boolean;
  createCollection: (collection: CreateCollectionInput) => Promise<void>;

  isDeleteCollectionLoading: boolean;
  deleteCollection: (collectionId: string) => Promise<void>;

  isCreateCollectionEntryLoading: boolean;
  createCollectionEntry: ({ collectionId, input }: { collectionId: string; input: CreateEntryInput }) => Promise<void>;

  isDeleteCollectionEntryLoading: boolean;
  deleteCollectionEntry: ({ collectionId, input }: { collectionId: string; input: DeleteEntryInput }) => Promise<void>;

  areCollectionsStreaming: boolean;
  streamedCollections: StreamCollectionsEvent[];
  startCollectionsStream: () => void;

  areCollectionEntriesStreaming: boolean;
  streamedCollectionEntries: EntryInfo[];
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

  isCreateCollectionLoading: false,
  createCollection: async (collection) => {
    set(() => ({
      isCreateCollectionLoading: true,
    }));

    const result = await invokeTauriIpc<CreateCollectionOutput>("create_collection", { input: collection });

    if (result.status === "error") {
      throw new Error(String(result.error));
    }

    set((state) => ({
      streamedCollections: [
        ...state.streamedCollections,
        {
          id: result.data.id,
          name: collection.name,
          order: collection.order ?? null,
        },
      ],
    }));

    set(() => ({
      isCreateCollectionLoading: false,
    }));
  },

  isDeleteCollectionLoading: false,
  deleteCollection: async (collectionId) => {
    set(() => ({
      isDeleteCollectionLoading: true,
    }));

    const result = await invokeTauriIpc<DeleteCollectionOutput>("delete_collection", { input: { id: collectionId } });

    if (result.status === "error") {
      throw new Error(String(result.error));
    }

    set((state) => ({
      streamedCollections: state.streamedCollections.filter((c) => c.id !== collectionId),
    }));

    set(() => ({
      isDeleteCollectionLoading: false,
    }));
  },

  isCreateCollectionEntryLoading: false,
  createCollectionEntry: async ({ collectionId, input }) => {
    set(() => ({
      isCreateCollectionEntryLoading: true,
    }));

    const result = await invokeTauriIpc<CreateEntryOutput>("create_collection_entry", {
      collectionId,
      input,
    });

    if (result.status === "error") {
      throw new Error(String(result.error));
    }

    if ("dir" in input) {
      let entryClass: "Request" | "Endpoint" | "Component" | "Schema" = "Request";
      if ("request" in input.dir.configuration) {
        entryClass = "Request";
      } else if ("endpoint" in input.dir.configuration) {
        entryClass = "Endpoint";
      } else if ("component" in input.dir.configuration) {
        entryClass = "Component";
      } else if ("schema" in input.dir.configuration) {
        entryClass = "Schema";
      }

      set((state) => ({
        streamedCollectionEntries: [
          ...state.streamedCollectionEntries,
          {
            id: result.data.id,
            name: input.dir.name,
            order: input.dir.order ?? undefined,
            path: `${input.dir.path.replaceAll("/", "")}\\${input.dir.name}`,
            class: entryClass,
            kind: "Dir" as const,
            expanded: false,
          },
        ],
      }));
    } else if ("item" in input) {
      let entryClass: "Request" | "Endpoint" | "Component" | "Schema" = "Request";
      let protocol: "Get" | "Post" | "Put" | "Delete" | "WebSocket" | "Graphql" | "Grpc" | undefined = undefined;

      if ("request" in input.item.configuration) {
        entryClass = "Request";
        if ("http" in input.item.configuration.request) {
          const method = input.item.configuration.request.http.requestParts.method;
          if (method === "GET") protocol = "Get";
          else if (method === "POST") protocol = "Post";
          else if (method === "PUT") protocol = "Put";
          else if (method === "DELETE") protocol = "Delete";
        }
      } else if ("endpoint" in input.item.configuration) {
        entryClass = "Endpoint";
        protocol = "Get";
      } else if ("component" in input.item.configuration) {
        entryClass = "Component";
      } else if ("schema" in input.item.configuration) {
        entryClass = "Schema";
      }

      set((state) => ({
        streamedCollectionEntries: [
          ...state.streamedCollectionEntries,
          {
            id: result.data.id,
            name: input.item.name,
            order: input.item.order ?? undefined,
            path: `${input.item.path.replaceAll("/", "")}\\${input.item.name}`,
            class: entryClass,
            kind: "Item" as const,
            protocol,
            expanded: false,
          },
        ],
      }));
    }

    set(() => ({
      isCreateCollectionEntryLoading: false,
    }));
  },

  isDeleteCollectionEntryLoading: false,
  deleteCollectionEntry: async ({ collectionId, input }) => {
    set(() => ({
      isDeleteCollectionEntryLoading: true,
    }));

    const result = await invokeTauriIpc<DeleteEntryOutput>("delete_collection_entry", {
      collectionId,
      input,
    });

    if (result.status === "error") {
      throw new Error(String(result.error));
    }

    set((state) => ({
      streamedCollectionEntries: state.streamedCollectionEntries.filter((c) => c.id !== input.id),
      isDeleteCollectionEntryLoading: false,
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
        console.log("onCollectionEvent", collection);
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
  startCollectionEntriesStream: async (collectionId) => {
    try {
      set({ areCollectionEntriesStreaming: true });

      const onCollectionEntryEvent = new Channel<EntryInfo>();

      onCollectionEntryEvent.onmessage = (collectionEntry) => {
        console.log("onCollectionEntryEvent", collectionEntry);
        set((state) => {
          const existingCollectionEntry = state.streamedCollectionEntries.find((c) => c.id === collectionEntry.id);
          if (existingCollectionEntry) {
            return state;
          }
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
