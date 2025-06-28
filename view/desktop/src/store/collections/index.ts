import { create } from "zustand";

import { CollectionTree, TreeCollectionNode, TreeCollectionRootNode } from "@/components/CollectionTree/types";
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
import { join, sep } from "@tauri-apps/api/path";

import AzureDevOpsTestCollection from "../../assets/AzureDevOpsTestCollection.json";
import SapicTestCollection from "../../assets/SapicTestCollection.json";
import WhatsAppBusinessTestCollection from "../../assets/WhatsAppBusinessTestCollection.json";
import { getClassAndProtocolFromEntyInput } from "./utils/getClassAndProtocolFromEntyInput";

export interface CollectionsStoreState {
  collections: CollectionTree[];
  setCollections: (collections: CollectionTree[]) => void;
  expandAll: () => void;
  collapseAll: () => void;
  updateCollection: (collection: CollectionTree) => void;

  collectionsTrees: TreeCollectionRootNode[];
  setCollectionsTrees: (collectionsTrees: TreeCollectionRootNode[]) => void;
  updateCollectionTree: (collectionsTree: TreeCollectionRootNode) => void;

  isCreateCollectionLoading: boolean;
  createCollection: (collection: CreateCollectionInput) => Promise<CreateCollectionOutput>;

  isDeleteCollectionLoading: boolean;
  deleteCollection: (collectionId: string) => Promise<void>;

  isCreateCollectionEntryLoading: boolean;
  createCollectionEntry: ({
    collectionId,
    input,
  }: {
    collectionId: string;
    input: CreateEntryInput;
  }) => Promise<EntryInfo | null>;

  isDeleteCollectionEntryLoading: boolean;
  deleteCollectionEntry: ({ collectionId, input }: { collectionId: string; input: DeleteEntryInput }) => Promise<void>;

  areCollectionsStreaming: boolean;
  streamedCollections: StreamCollectionsEvent[];
  startCollectionsStream: () => void;

  areCollectionEntriesStreaming: boolean;
  streamedCollectionEntries: EntryInfo[];
  startCollectionEntriesStream: (collection: StreamCollectionsEvent) => void;
  distributeEntryToCollectionTree: (entry: EntryInfo, collectionId: string) => void;
  updateStreamedCollection: (collection: StreamCollectionsEvent) => void;
}

export const useCollectionsStore = create<CollectionsStoreState>((set, get) => ({
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
          tree: expandAllNodes(collection.tree as TreeCollectionNode),
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
          tree: collapseAllNodes(collection.tree as TreeCollectionNode),
        };
      }),
    }));
  },
  updateCollection: (updatedCollection: CollectionTree) => {
    set((state) => ({
      collections: state.collections.map((c) => (c.id === updatedCollection.id ? { ...updatedCollection } : c)),
    }));
  },

  //streamed collections

  collectionsTrees: [],
  setCollectionsTrees: (collectionsTrees: TreeCollectionRootNode[]) => {
    set({ collectionsTrees });
  },
  updateCollectionTree: (collectionsTree: TreeCollectionRootNode) => {
    set((state) => ({
      collectionsTrees: state.collectionsTrees.map((c) => (c.id === collectionsTree.id ? collectionsTree : c)),
    }));
  },

  isCreateCollectionLoading: false,
  createCollection: async (collection: CreateCollectionInput): Promise<CreateCollectionOutput> => {
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
          picturePath: null,
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
            id: Math.random().toString(),
            name: "endpoints",
            path: {
              raw: "endpoints",
              segments: ["endpoints"],
            },
            class: "Endpoint",
            kind: "Dir",
            expanded: true,
            childNodes: [],
          },
          schemas: {
            id: Math.random().toString(),
            name: "schemas",
            path: {
              raw: "schemas",
              segments: ["schemas"],
            },
            class: "Schema",
            kind: "Dir",
            expanded: true,
            childNodes: [],
          },
          components: {
            id: Math.random().toString(),
            name: "components",
            path: {
              raw: "components",
              segments: ["components"],
            },
            class: "Component",
            kind: "Dir",
            expanded: true,
            childNodes: [],
          },
          requests: {
            id: Math.random().toString(),
            name: "requests",
            path: {
              raw: "requests",
              segments: ["requests"],
            },
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

    return result.data;
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

    const { entryClass, protocol } = getClassAndProtocolFromEntyInput(input);

    if ("dir" in input) {
      const rawpath = await join(input.dir.path, input.dir.name);

      const newEntry: EntryInfo = {
        id: result.data.id,
        name: input.dir.name,
        order: input.dir.order ?? get().streamedCollectionEntries.length + 1,
        path: {
          raw: rawpath,
          segments: rawpath.split(sep()),
        },
        class: entryClass,
        kind: "Dir" as const,
        expanded: false,
      };

      set((state) => ({
        streamedCollectionEntries: [...state.streamedCollectionEntries, newEntry],
      }));

      get().distributeEntryToCollectionTree(newEntry, collectionId);

      set(() => ({
        isCreateCollectionEntryLoading: false,
      }));

      return newEntry;
    } else if ("item" in input) {
      const rawpath = await join(input.item.path, input.item.name);

      const newEntry: EntryInfo = {
        id: result.data.id,
        name: input.item.name,
        order: input.item.order ?? undefined,
        path: {
          raw: rawpath,
          segments: rawpath.split(sep()),
        },
        class: entryClass,
        kind: "Item" as const,
        protocol,
        expanded: false,
      };

      set((state) => ({
        streamedCollectionEntries: [...state.streamedCollectionEntries, newEntry],
      }));

      get().distributeEntryToCollectionTree(newEntry, collectionId);

      set(() => ({
        isCreateCollectionEntryLoading: false,
      }));

      return newEntry;
    }

    set(() => ({
      isCreateCollectionEntryLoading: false,
    }));

    return null;
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
      collectionsTrees: state.collectionsTrees.map((c) => filterOutNodeFromCollectionTree(c, input)),
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
                  id: Math.random().toString(),
                  name: "endpoints",
                  path: {
                    raw: "endpoints",
                    segments: ["endpoints"],
                  },
                  class: "Endpoint",
                  kind: "Dir",
                  expanded: false,
                  childNodes: [],
                },
                schemas: {
                  id: Math.random().toString(),
                  name: "schemas",
                  path: {
                    raw: "schemas",
                    segments: ["schemas"],
                  },
                  class: "Schema",
                  kind: "Dir",
                  expanded: false,
                  childNodes: [],
                },
                components: {
                  id: Math.random().toString(),
                  name: "components",
                  path: {
                    raw: "components",
                    segments: ["components"],
                  },
                  class: "Component",
                  kind: "Dir",
                  expanded: false,
                  childNodes: [],
                },
                requests: {
                  id: Math.random().toString(),
                  name: "requests",
                  path: {
                    raw: "requests",
                    segments: ["requests"],
                  },
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

      // console.log("collectionsTrees", get().collectionsTrees);
    } catch (error) {
      console.error("Failed to get collection entries:", error);
    } finally {
      set({ areCollectionEntriesStreaming: false });
    }
  },

  distributeEntryToCollectionTree: async (entry, collectionId) => {
    console.log("distributeEntryToCollectionTree", entry);
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

    const category = categoryMap[entry.class];
    if (!category) {
      console.error(`Unknown class ${entry.class}`);
      return;
    }

    // Get the root node for this category
    const rootNode = (collection as TreeCollectionRootNode)[category] as TreeCollectionNode;
    if (!rootNode) {
      console.error(`Category ${category} not found in collection`);
      return;
    }

    // If path has one component (root node)
    if (entry.path.segments.length === 1) {
      // Update root node properties, preserving childNodes
      const existingChildNodes = rootNode.childNodes;
      Object.assign(rootNode, entry, { childNodes: existingChildNodes });

      // Update state
      set((state) => ({
        collectionsTrees: state.collectionsTrees.map((c) => (c.id === collectionId ? collection : c)),
      }));
      return;
    }

    // Handle nested paths
    let currentNode = rootNode;
    const relativePath = entry.path.segments.slice(1);
    // console.log("relativePath", entry.path.segments, relativePath);

    // Navigate or create intermediate directories
    for (let i = 0; i < relativePath.length - 1; i++) {
      const component = relativePath[i];

      const rawpath = await join(currentNode.path.raw, component);
      let child = currentNode.childNodes.find((node) => node.name === component && node.kind === "Dir");

      if (!child) {
        child = {
          id: Math.random().toString(), // Generate unique ID for intermediate directory
          name: component,
          path: {
            raw: rawpath,
            segments: rawpath.split(sep()),
          },
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
      const existingChildNodes = existingNode.childNodes;
      Object.assign(existingNode, entry, { childNodes: existingChildNodes });
    } else {
      // Create new node
      const rawpath = await join(currentNode.path.raw, lastComponent);
      const newNode: TreeCollectionNode = {
        ...entry,
        path: {
          raw: rawpath,
          segments: rawpath.split(sep()),
        },
        childNodes: [],
      };

      currentNode.childNodes.push(newNode);
    }

    // Update state to trigger re-render
    set((state) => ({
      collectionsTrees: state.collectionsTrees.map((c) => (c.id === collectionId ? collection : c)),
    }));
  },
  //TODO: this is temporary, we need to update the collection in the backend too
  updateStreamedCollection: (collection: StreamCollectionsEvent) => {
    set((state) => ({
      streamedCollections: state.streamedCollections.map((c) => (c.id === collection.id ? collection : c)),
      collectionsTrees: state.collectionsTrees.map((c) =>
        c.id === collection.id ? { ...c, name: collection.name } : c
      ),
    }));
  },
}));

const filterOutNodeFromCollectionTree = (tree: TreeCollectionRootNode, input: DeleteEntryInput) => {
  return {
    ...tree,
    requests: filterOutNodeFromNode(tree.requests, input.id),
    endpoints: filterOutNodeFromNode(tree.endpoints, input.id),
    components: filterOutNodeFromNode(tree.components, input.id),
    schemas: filterOutNodeFromNode(tree.schemas, input.id),
  };
};

const filterOutNodeFromNode = (node: TreeCollectionNode, id: string): TreeCollectionNode => {
  return {
    ...node,
    childNodes: node.childNodes
      .filter((childNode) => childNode.id !== id)
      .map((childNode) => filterOutNodeFromNode(childNode, id)),
  };
};
