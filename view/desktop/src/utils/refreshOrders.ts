export const refreshOrders = <T extends { order: number }>(data: T[]) => {
  return data.map((row, index) => ({ ...row, order: index + 1 }));
};
