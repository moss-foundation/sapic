import { useCollectionsStore } from "@/store/collections";
import { Collection } from "@/components/CollectionTree/types";

// For now, we'll use the first collection as the active one
// This should be replaced with proper collection selection logic
export const useActiveCollection = (): Collection | null => {
  const { collections } = useCollectionsStore();

  // Return the first collection for now (mock implementation)
  // In the future, this should track the actual selected collection
  return collections.length > 0 ? collections[0] : null;
};
