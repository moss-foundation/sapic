export const sortObjectsByOrder = <
  T extends {
    id: any;
    order?: number | null;
  },
>(
  objects: T[],
  nameKey?: keyof T
) => {
  return [...objects].sort((a, b) => {
    const aHasOrder = a.order !== undefined && a.order !== null;
    const bHasOrder = b.order !== undefined && b.order !== null;

    if (aHasOrder && bHasOrder) return a.order! - b.order!;
    if (aHasOrder && !bHasOrder) return -1;
    if (!aHasOrder && bHasOrder) return 1;

    // Both without order — sort alphabetically by nameKey if provided
    if (nameKey != null) {
      const aVal = a[nameKey];
      const bVal = b[nameKey];
      const aStr = typeof aVal === "string" ? aVal : String(aVal ?? "");
      const bStr = typeof bVal === "string" ? bVal : String(bVal ?? "");
      return aStr.localeCompare(bStr);
    }
    return 0;
  });
};
