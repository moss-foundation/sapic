export const defaultStates = {
  Console: 1,
  Trash: 2,
  Cookie: 3,
} as const satisfies Record<string, number>;
