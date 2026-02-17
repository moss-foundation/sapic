interface OrderableItem {
  id: string;
  name: string;
  order?: number | null;
}

/**
 * Given a list of items, computes a canonical order:
 * - Items with existing orders keep their relative order
 * - Items without orders are placed after, sorted alphabetically by name
 * - Returns only items whose order actually changed
 */
export const computeOrderUpdates = (items: OrderableItem[]): Record<string, number> => {
  const withOrders = items.filter((i) => i.order != null).sort((a, b) => a.order! - b.order!);

  const withoutOrders = items.filter((i) => i.order == null).sort((a, b) => a.name.localeCompare(b.name));

  const sorted = [...withOrders, ...withoutOrders];
  const updates: Record<string, number> = {};

  sorted.forEach((item, index) => {
    const newOrder = index + 1;
    if (item.order !== newOrder) {
      updates[item.id] = newOrder;
    }
  });

  return updates;
};

/**
 * Assigns sequential orders (1..N) to items in their given array order.
 * Returns only items whose order actually changed.
 */
export const computeSequentialOrders = (items: { id: string; order?: number | null }[]): Record<string, number> => {
  const updates: Record<string, number> = {};

  items.forEach((item, index) => {
    const newOrder = index + 1;
    if (item.order !== newOrder) {
      updates[item.id] = newOrder;
    }
  });

  return updates;
};
