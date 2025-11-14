interface WithId {
  id: string | number;
}

export const swapObjectsById = <T extends WithId>(obj1: T, obj2: T, list: T[]): T[] => {
  const newList = [...list];

  const index1 = newList.findIndex((item) => item.id === obj1.id);
  const index2 = newList.findIndex((item) => item.id === obj2.id);

  if (index1 === -1 || index2 === -1) {
    throw new Error("One or both objects were not found in the newList.");
  }

  [newList[index1], newList[index2]] = [newList[index2], newList[index1]];

  return newList;
};
