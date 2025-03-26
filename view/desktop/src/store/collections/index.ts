import { create } from "zustand";

import { Collection } from "@/components/Tree/types";

import AzureDevOpsTestCollection from "../../assets/AzureDevOpsTestCollection.json";
import SapicTestCollection from "../../assets/SapicTestCollection.json";
import WhatsAppBusinessTestCollection from "../../assets/WhatsAppBusinessTestCollection.json";

interface CollectionsStoreState {
  collections: Collection[];
  setCollections: (collections: Collection[]) => void;
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
}));
