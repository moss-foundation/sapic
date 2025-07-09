//TODO order should always be set, it's a temporary solution until backend updates it's type
export const sortByOrder = <T extends { order?: number }>(entries: T[]) => {
  return [...entries].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
};
