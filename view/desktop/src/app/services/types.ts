export type TypedValue<T> = T extends object | boolean | number | string | null ? T : never;
