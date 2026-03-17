interface MovableItem {
  id: string;
  order?: number | null;
}

interface MoveItemInArrayProps<T extends MovableItem> {
  arr: T[];
  itemToMove: T;
  toIndex: number;
}

export const moveItemInArray = <T extends MovableItem>({ arr, itemToMove, toIndex }: MoveItemInArrayProps<T>) => {
  const moved = [
    ...arr.slice(0, toIndex).filter((p) => p.id !== itemToMove.id),
    itemToMove,
    ...arr.slice(toIndex).filter((p) => p.id !== itemToMove.id),
  ];

  const updatedOrders = moved.map((item, index) => ({
    ...item,
    order: index + 1,
  }));

  return updatedOrders;
};
