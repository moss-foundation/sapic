import { create } from "zustand";

import {
  checkIfTreeIsCollapsed,
  checkIfTreeIsExpanded,
  collapseAllNodes,
  expandAllNodes,
} from "@/components/CollectionTree/utils";
import { CollectionTree, TreeCollectionNode, TreeCollectionRootNode } from "@/components/CollectionTreeV2/types";
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
  collectionsTrees: TreeCollectionRootNode[];
  setCollectionsTrees: (collectionsTrees: TreeCollectionRootNode[]) => void;
  updateCollectionTree: (collectionsTree: TreeCollectionRootNode) => void;

  collections: CollectionTree[];
  setCollections: (collections: CollectionTree[]) => void;
  expandAll: () => void;
  collapseAll: () => void;
  updateCollection: (collection: CollectionTree) => void;

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
  startCollectionEntriesStream: (collection: StreamCollectionsEvent) => void;
  distributeEntryToCollections: (entry: EntryInfo, collectionId: string) => void;
}

export const useCollectionsStore = create<CollectionsStoreState>((set, get) => ({
  collectionsTrees: [],
  setCollectionsTrees: (collectionsTrees: TreeCollectionRootNode[]) => {
    set({ collectionsTrees });
  },
  updateCollectionTree: (collectionsTree: TreeCollectionRootNode) => {
    set((state) => ({
      collectionsTrees: state.collectionsTrees.map((c) => (c.id === collectionsTree.id ? { ...collectionsTree } : c)),
    }));
  },
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

  isCreateCollectionLoading: false,
  createCollection: async (collection) => {
    set(() => ({
      isCreateCollectionLoading: true,
    }));

    const result = await invokeTauriIpc<CreateCollectionOutput>("create_collection", { input: collection });

    if (result.status === "error") {
      set(() => ({
        isCreateCollectionLoading: false,
      }));
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
      set(() => ({
        isCreateCollectionEntryLoading: false,
      }));
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
      set(() => ({
        isDeleteCollectionEntryLoading: false,
      }));
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
        collectionsTrees: [],
      });

      const onCollectionEvent = new Channel<StreamCollectionsEvent>();
      onCollectionEvent.onmessage = (collection) => {
        console.log("collection", collection);
        set((state) => {
          const existingCollection = state.streamedCollections.find((c) => c.id === collection.id);
          if (existingCollection) {
            return state;
          }
          return {
            ...state,
            streamedCollections: [...state.streamedCollections, collection],
            collectionsTrees: [
              ...state.collectionsTrees,
              {
                id: collection.id,
                name: collection.name,
                order: collection.order,
                expanded: true,
                Endpoints: {
                  id: "Endpoints-id",
                  name: "Endpoints",
                  order: 1,
                  path: "Endpoints",
                  class: "Endpoint",
                  kind: "Dir",
                  expanded: true,
                  childNodes: [],
                },
                Schemas: {
                  id: "Schemas-id",
                  name: "Schemas",
                  order: 2,
                  path: "Schemas",
                  class: "Schema",
                  kind: "Dir",
                  expanded: true,
                  childNodes: [],
                },
                Components: {
                  id: "Components-id",
                  name: "Components",
                  order: 3,
                  path: "Components",
                  class: "Component",
                  kind: "Dir",
                  expanded: true,
                  childNodes: [],
                },
                Requests: {
                  id: "Requests-id",
                  name: "Requests",
                  order: 4,
                  path: "Requests",
                  class: "Request",
                  kind: "Dir",
                  expanded: true,
                  childNodes: [],
                },
              },
            ],
          };
        });

        get().startCollectionEntriesStream(collection);
      };

      await invokeTauriIpc("stream_collections", {
        channel: onCollectionEvent,
      });

      // console.log("streamedCollections", get().streamedCollections);
    } catch (error) {
      console.error("Failed to set up stream_collections:", error);
    } finally {
      set({ areCollectionsStreaming: false });
    }
  },

  areCollectionEntriesStreaming: false,
  streamedCollectionEntries: [],
  startCollectionEntriesStream: async (collection) => {
    // console.log("startCollectionEntriesStream");
    try {
      set({ areCollectionEntriesStreaming: true });

      const onCollectionEntryEvent = new Channel<EntryInfo>();

      onCollectionEntryEvent.onmessage = (collectionEntry) => {
        console.log("collectionEntry", collectionEntry);
        set((state) => {
          const existingCollectionEntry = state.streamedCollectionEntries.find((c) => c.id === collectionEntry.id);
          if (existingCollectionEntry) {
            return state;
          }

          get().distributeEntryToCollections(collectionEntry, collection.id);
          return {
            ...state,
            streamedCollectionEntries: [...state.streamedCollectionEntries, collectionEntry],
          };
        });
      };

      await invokeTauriIpc("stream_collection_entries", {
        collectionId: collection.id,
        channel: onCollectionEntryEvent,
      });
    } catch (error) {
      console.error("Failed to get collection entries:", error);
    } finally {
      set({ areCollectionEntriesStreaming: false });
    }
  },

  distributeEntryToCollections(entry: EntryInfo, collectionId: string) {
    const collection = get().collectionsTrees.find((c) => c.id === collectionId);
    if (!collection) {
      console.error("Collection not found:", collectionId);
      return;
    }

    let category: TreeCollectionNode;
    switch (entry.class) {
      case "Request":
        category = collection.Requests;
        break;
      case "Endpoint":
        category = collection.Endpoints;
        break;
      case "Component":
        category = collection.Components;
        break;
      case "Schema":
        category = collection.Schemas;
        break;
      default:
        console.error("Invalid entry class:", entry.class);
        return;
    }

    const parts = entry.path.split("\\");
    const expectedFirstPart = entry.class.toLowerCase() + "s"; // e.g., "requests"
    if (parts[0] !== expectedFirstPart) {
      console.error("Path does not start with expected category:", entry.path);
      return;
    }

    const pathParts = parts.slice(1);
    if (pathParts.length === 0) {
      console.error("Entry path is too short:", entry.path);
      return;
    }

    const parentPathParts = pathParts.slice(0, -1);
    let currentNode = category;
    for (const part of parentPathParts) {
      const childNode = currentNode.childNodes.find((node) => node.name === part && node.kind === "Dir");
      if (!childNode) {
        console.error("Parent directory not found:", part);
        return;
      }
      currentNode = childNode;
    }

    const newNode: TreeCollectionNode = {
      id: entry.id,
      name: entry.name,
      path: `\\${category.name}\\${pathParts.join("\\")}`,
      class: entry.class,
      kind: entry.kind,
      protocol: entry.protocol || undefined,
      order: entry.order || undefined,
      expanded: entry.expanded,
      childNodes: [],
    };

    currentNode.childNodes.push(newNode);

    // Update the state to trigger a re-render
    set((state) => ({ collectionsTrees: [...state.collectionsTrees] }));
  },
}));
