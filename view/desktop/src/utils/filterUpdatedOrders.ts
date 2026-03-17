interface OrderableItem {
  id: string;
  order?: number | null;
}

export const filterUpdatedOrders = <T extends OrderableItem>(original: T[], reordered: T[]): Record<string, number> => {
  const updatedOrders = reordered.reduce(
    (acc, item, index) => {
      const originalItem = original.find((p) => p.id === item.id);
      if (originalItem?.order !== index + 1) {
        acc[item.id] = index + 1;
      }
      return acc;
    },
    {} as Record<string, number>
  );

  return updatedOrders;
};
