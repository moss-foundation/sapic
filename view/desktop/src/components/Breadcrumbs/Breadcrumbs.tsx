import { useMemo, useState } from "react";

import { useCollectionsStore } from "@/store/collections";
import { useDockviewStore } from "@/store/Dockview";

import { DropdownMenu } from "..";
import Icon from "../Icon";
import { NodeProps } from "../Tree/types";
import { findNodeById } from "../Tree/utils";
import { BreadcrumbTree } from "./BreadcrumbTree";

export const Breadcrumbs = () => {
  const { currentActivePanelId } = useDockviewStore();
  const { collections } = useCollectionsStore();
  const [activeTree, setActiveTree] = useState<NodeProps | null>(null);

  const path = useMemo(() => {
    if (!currentActivePanelId) return [];

    const target = String(currentActivePanelId);
    for (const collection of collections) {
      const newPath = findPath(collection.tree, target);
      if (newPath) {
        setActiveTree(collection.tree);
        return newPath;
      }
    }

    setActiveTree(null);
    return [];
  }, [collections, currentActivePanelId]);

  return (
    <div className="flex items-center gap-1 px-3 py-[5px] text-[#6F6F6F]">
      {path.map((pathNode, index) => {
        if (!activeTree) return null;

        const node = findNodeById(activeTree, pathNode)!;

        return (
          <div key={pathNode} className="flex items-center">
            <DropdownMenu.Root>
              <DropdownMenu.Trigger className="cursor-pointer hover:underline">{pathNode}</DropdownMenu.Trigger>
              <DropdownMenu.Content>
                <BreadcrumbTree tree={node} onNodeClickCallback={() => {}} />
              </DropdownMenu.Content>
            </DropdownMenu.Root>

            {index !== path.length - 1 && (
              <span>
                <Icon icon="TreeChevronRightIcon" />
              </span>
            )}
          </div>
        );
      })}
    </div>
  );
};

const findPath = (node: NodeProps, target: string): string[] | null => {
  if (node.id === target) return [node.id];

  if (node.childNodes && node.childNodes.length > 0) {
    for (const child of node.childNodes) {
      const path = findPath(child, target);
      if (path) return [node.id.toString(), ...path];
    }
  }

  return null;
};

export default Breadcrumbs;
