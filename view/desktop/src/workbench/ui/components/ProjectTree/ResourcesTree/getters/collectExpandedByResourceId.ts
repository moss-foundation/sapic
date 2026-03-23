import type { ResourceNode } from "../types";

export function collectExpandedByResourceId(root: ResourceNode): Map<string, boolean> {
  const out = new Map<string, boolean>();
  const visit = (n: ResourceNode) => {
    out.set(n.id, n.expanded);
    for (const child of n.childNodes) {
      visit(child);
    }
  };
  visit(root);
  return out;
}
