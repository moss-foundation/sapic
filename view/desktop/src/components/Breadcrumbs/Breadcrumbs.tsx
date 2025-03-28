import { useMemo, useState } from "react";

import { useCollectionsStore } from "@/store/collections";

import { DropdownMenu, Icon } from "..";
import { TestCollectionIcon } from "../Tree/TestCollectionIcon";
import { NodeProps } from "../Tree/types";
import { findNodeById } from "../Tree/utils";
import { BreadcrumbTree } from "./BreadcrumbTree";

export const Breadcrumbs = ({ panelId }: { panelId: string }) => {
  const { collections } = useCollectionsStore();
  const [activeTree, setActiveTree] = useState<NodeProps | null>(null);

  const path = useMemo(() => {
    if (!panelId) return [];

    const target = String(panelId);
    for (const collection of collections) {
      const newPath = findPath(collection.tree, target);
      if (newPath) {
        setActiveTree(collection.tree);
        return newPath;
      }
    }

    setActiveTree(null);
    return [];
  }, [collections, panelId]);

  if (!activeTree) return null;

  return (
    <div className="flex items-center gap-1 px-3 py-[5px] text-[#6F6F6F]">
      {path.map((pathNode, index) => {
        const node = findNodeById(activeTree, pathNode)!;
        const lastItem = index === path.length - 1;

        if (lastItem) {
          return (
            <div key={pathNode} className="flex items-center gap-1">
              <TestCollectionIcon type={node.type} className="size-4.5" />
              <span>{pathNode}</span>
            </div>
          );
        }

        return (
          <div key={pathNode} className="flex items-center">
            <DropdownMenu.Root>
              <DropdownMenu.Trigger className="cursor-pointer hover:underline">{pathNode}</DropdownMenu.Trigger>
              <DropdownMenu.Content align="start">
                <BreadcrumbTree tree={node} onNodeClickCallback={() => {}} />
              </DropdownMenu.Content>
            </DropdownMenu.Root>
            {!lastItem && (
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
