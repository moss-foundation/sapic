import { create } from "zustand";

import { TreeCollectionNode, TreeCollectionRootNode } from "@/components/CollectionTree/types";
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

export interface CollectionsStoreState {
  collectionsTrees: TreeCollectionRootNode[];
  setCollectionsTrees: (collectionsTrees: TreeCollectionRootNode[]) => void;
  updateCollectionTree: (collectionsTree: TreeCollectionRootNode) => void;

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
  distributeEntryToCollectionTree: (entry: EntryInfo, collectionId: string) => void;
  updateEntry: (updatedEntry: EntryInfo, collectionId: string) => void;
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
          ...collection,
          order: collection.order ?? state.streamedCollections.length + 1,
        },
      ],
      collectionsTrees: [
        ...state.collectionsTrees,
        {
          id: result.data.id,
          name: collection.name,
          order: collection.order ?? state.streamedCollections.length + 1,
          expanded: true,
          endpoints: {
            id: "",
            name: "endpoints",
            path: "endpoints",
            class: "Endpoint",
            kind: "Dir",
            expanded: true,
            childNodes: [],
          },
          schemas: {
            id: "",
            name: "schemas",
            path: "schemas",
            class: "Schema",
            kind: "Dir",
            expanded: true,
            childNodes: [],
          },
          components: {
            id: "",
            name: "components",
            path: "components",
            class: "Component",
            kind: "Dir",
            expanded: true,
            childNodes: [],
          },
          requests: {
            id: "",
            name: "requests",
            path: "requests",
            class: "Request",
            kind: "Dir",
            expanded: false,
            childNodes: [],
          },
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
      set(() => ({
        isDeleteCollectionLoading: false,
      }));
      throw new Error(String(result.error));
    }

    set((state) => ({
      streamedCollections: state.streamedCollections.filter((c) => c.id !== collectionId),
      collectionsTrees: state.collectionsTrees.filter((c) => c.id !== collectionId),
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
            order: input.dir.order ?? state.streamedCollectionEntries.length + 1,
            path: `${input.dir.path.replaceAll("/", "")}\\${input.dir.name}`,
            class: entryClass,
            kind: "Dir" as const,
            expanded: false,
          },
        ],
      }));

      get().distributeEntryToCollectionTree(
        {
          id: result.data.id,
          name: input.dir.name,
          order: input.dir.order ?? undefined,
          path: `${input.dir.path.replaceAll("/", "")}\\${input.dir.name}`,
          class: entryClass,
          kind: "Dir" as const,
          expanded: false,
        },
        collectionId
      );
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

      get().distributeEntryToCollectionTree(
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
        collectionId
      );
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
        // console.log("collection", collection);
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
                endpoints: {
                  id: "",
                  name: "endpoints",
                  path: "endpoints",
                  class: "Endpoint",
                  kind: "Dir",
                  expanded: true,
                  childNodes: [],
                },
                schemas: {
                  id: "",
                  name: "schemas",
                  path: "schemas",
                  class: "Schema",
                  kind: "Dir",
                  expanded: false,
                  childNodes: [],
                },
                components: {
                  id: "",
                  name: "components",
                  path: "components",
                  class: "Component",
                  kind: "Dir",
                  expanded: true,
                  childNodes: [],
                },
                requests: {
                  id: "",
                  name: "requests",
                  path: "requests",
                  class: "Request",
                  kind: "Dir",
                  expanded: false,
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

      console.log("collectionsTrees", get().collectionsTrees);
    } catch (error) {
      console.error("Failed to set up stream_collections:", error);
    } finally {
      set({ areCollectionsStreaming: false });
    }
  },

  areCollectionEntriesStreaming: false,
  streamedCollectionEntries: [],
  startCollectionEntriesStream: async (collection) => {
    try {
      set({ areCollectionEntriesStreaming: true });

      const onCollectionEntryEvent = new Channel<EntryInfo>();

      onCollectionEntryEvent.onmessage = (collectionEntry) => {
        // console.log("collectionEntry", collectionEntry);
        set((state) => {
          const existingCollectionEntry = state.streamedCollectionEntries.find((c) => c.id === collectionEntry.id);
          if (existingCollectionEntry) {
            return state;
          }

          get().distributeEntryToCollectionTree(collectionEntry, collection.id);
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

  distributeEntryToCollectionTree: (entry, collectionId) => {
    // Find the collection by ID
    const collection = get().collectionsTrees.find((col) => col.id === collectionId);
    if (!collection) {
      console.error(`Collection with ID ${collectionId} not found`);
      return;
    }

    // Map entry class to collection category
    const categoryMap: { [key: string]: string } = {
      "Request": "requests",
      "Endpoint": "endpoints",
      "Component": "components",
      "Schema": "schemas",
    };

    //TODO: uncomment this when backend is fixed
    // const category = categoryMap[entry.class];
    // if (!category) {
    //   console.error(`Unknown class ${entry.class}`);
    //   return;
    // }

    const category = entry.path.split("\\")[0];
    if (!category) {
      console.error(`Unknown class ${category} ${entry.path}`);
      return;
    }
    // Get the root node for this category
    const rootNode = (collection as TreeCollectionRootNode)[category] as TreeCollectionNode;
    if (!rootNode) {
      console.error(`Category ${category} not found in collection`);
      return;
    }

    // Split the path into components
    const pathComponents = entry.path.split("\\");

    // If path has one component (root node)
    if (pathComponents.length === 1) {
      // Update root node properties, preserving childNodes
      Object.assign(rootNode, entry);
      return;
    }

    // Handle nested paths
    let currentNode = rootNode;
    const relativePath = pathComponents.slice(1);

    // Navigate or create intermediate directories
    for (let i = 0; i < relativePath.length - 1; i++) {
      const component = relativePath[i];
      let child = currentNode.childNodes.find((node) => node.name === component && node.kind === "Dir");

      if (!child) {
        child = {
          id: "",
          name: component,
          path: `${currentNode.path}\\${component}`,
          class: entry.class,
          kind: "Dir",
          protocol: undefined,
          order: undefined,
          expanded: false,
          childNodes: [],
        };
        currentNode.childNodes.push(child);
      }
      currentNode = child;
    }

    // Handle the final component
    const lastComponent = relativePath[relativePath.length - 1];
    const existingNode = currentNode.childNodes.find((node) => node.name === lastComponent);

    if (existingNode) {
      // Update existing node, preserving childNodes
      Object.assign(existingNode, entry);
    } else {
      // Create new node
      const newNode: TreeCollectionNode = {
        id: entry.id,
        name: entry.name,
        path: entry.path,
        class: entry.class,
        kind: entry.kind,
        protocol: entry.protocol,
        order: entry.order ? entry.order : currentNode.childNodes.length + 1,
        expanded: entry.expanded,
        childNodes: [],
      };
      currentNode.childNodes.push(newNode);
    }
  },

  updateEntry(updatedEntry, collectionId) {},
}));
