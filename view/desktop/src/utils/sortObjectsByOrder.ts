export const sortObjectsByOrder = <T extends { order?: number }>(objects: T[]) => {
  return [...objects].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
};
