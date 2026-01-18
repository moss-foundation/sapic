import { AddPanelOptions } from "moss-tabs";
import { ComponentType } from "react";

import { tabbedPaneComponents } from "@/workbench/ui/parts/TabbedPane/TabbedPaneComponents";

// 1. EXTRACT PARAMS
// We return 'Record<string, never>' (an empty object) instead of 'undefined'.
// This fixes the "Type 'P' does not satisfy constraint 'object'" error.
type ExtractParams<T> =
  T extends ComponentType<infer P>
    ? "params" extends keyof P
      ? P["params"]
      : Record<string, never>
    : Record<string, never>;

// 2. CHECK REQUIRED KEYS
// We check if an empty object (Record<string, never>) can be assigned to T.
// If yes, it means T has no required keys (all keys are optional or T is empty).
// If no, it means T has at least one required key.
type HasRequiredKeys<T> = Record<string, never> extends T ? false : true;

// 3. GENERATE TYPED OPTIONS
export type TypedAddPanelOptions = {
  [K in keyof typeof tabbedPaneComponents]: ExtractParams<(typeof tabbedPaneComponents)[K]> extends infer P
    ? // We explicitly check P extends object to silence any remaining constraint fears,
      // though ExtractParams handles this now.
      P extends object
      ? HasRequiredKeys<P> extends true
        ? // CASE: Params are REQUIRED (e.g. Endpoint)
          // We Omit 'params' from the base lib type so we can enforce it.
          Omit<AddPanelOptions<P>, "component" | "params"> & {
            component: K;
            params: P; // <--- Required
          }
        : // CASE: Params are OPTIONAL or EMPTY (e.g. Welcome)
          // We Omit 'params' to redefine it as optional.
          Omit<AddPanelOptions<P>, "component" | "params"> & {
            component: K;
            params?: P; // <--- Optional
          }
      : never
    : never;
}[keyof typeof tabbedPaneComponents];
