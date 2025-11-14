import { Icons } from "@/lib/ui";
import { ProjectTreeNode, ProjectTreeRootNode } from "@/workbench/ui/components/ProjectTree/types";

export const getFolderIcon = (): Icons => {
  return "Folder";
};

export const findNodeInProject = (project: ProjectTreeRootNode, searchId: string) => {
  // Search in all categories

  for (const category of project.childNodes) {
    if (category.id === searchId) return category;

    // Recursively search child nodes
    const findInChildren = (node: ProjectTreeNode): ProjectTreeNode | undefined => {
      if (node.id === searchId) return node;
      for (const child of node.childNodes) {
        const found = findInChildren(child);
        if (found) return found;
      }
      return undefined;
    };

    const found = findInChildren(category);
    if (found) return found;
  }
  return undefined;
};
