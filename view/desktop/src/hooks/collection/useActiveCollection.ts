import { CollectionTree } from "@/components/CollectionTree/types";
import { useCollectionsStore } from "@/store/collections";

// For now, we'll use the first collection as the active one
// This should be replaced with proper collection selection logic
export const useActiveCollection = (): CollectionTree | null => {
  const { collections } = useCollectionsStore();

  // Return the first collection for now (mock implementation)
  // In the future, this should track the actual selected collection
  return collections.length > 0 ? collections[0] : null;
};
