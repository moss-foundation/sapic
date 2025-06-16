import { useEffect, useState } from "react";

import { ActionMenu } from "@/components";
import { Icon } from "@/lib/ui";
import { useCollectionsStore } from "@/store/collections";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { TestCollectionIcon } from "../CollectionTree/TestCollectionIcon";
import { NodeProps } from "../CollectionTree/types";
import { findNodeById } from "../CollectionTree/utils";
import { BreadcrumbTree } from "./BreadcrumbTree";

export const Breadcrumbs = ({ panelId }: { panelId: string }) => {
  const { collections } = useCollectionsStore();
  const [activeTree, setActiveTree] = useState<NodeProps | null>(null);
  const { addOrFocusPanel } = useTabbedPaneStore();
  const [path, setPath] = useState<string[]>([]);

  useEffect(() => {
    if (!panelId) {
      setActiveTree(null);
      setPath([]);
      return;
    }

    const target = String(panelId);
    for (const collection of collections) {
      const newPath = findPath(collection.tree, target);
      if (newPath) {
        setActiveTree(collection.tree);
        setPath(newPath);
        return;
      }
    }

    setActiveTree(null);
    setPath([]);
  }, [collections, panelId]);

  if (!activeTree) return null;

  return (
    <div className="flex items-center justify-between px-2">
      <div className="flex w-max items-center gap-1 overflow-hidden text-[#6F6F6F] select-none">
        {path.map((pathNode, index) => {
          const node = findNodeById(activeTree, pathNode)!;
          const lastItem = index === path.length - 1;

          if (lastItem) {
            return (
              <div key={pathNode} className="contents">
                <TestCollectionIcon type={node.type} className="size-4" />
                <span className="min-w-max">{pathNode}</span>
              </div>
            );
          }

          return (
            <div key={pathNode} className="contents">
              <ActionMenu.Root>
                <ActionMenu.Trigger className="min-w-max cursor-pointer hover:underline">{pathNode}</ActionMenu.Trigger>
                <ActionMenu.Content align="start">
                  <BreadcrumbTree
                    tree={node}
                    onNodeClick={(node) => {
                      if (!node.isFolder)
                        addOrFocusPanel({
                          id: `${node.id}`,
                          params: {
                            iconType: node.type,
                            workspace: true,
                          },
                          component: "Default",
                        });
                    }}
                  />
                </ActionMenu.Content>
              </ActionMenu.Root>
              {!lastItem && (
                <span>
                  <Icon icon="ChevronRight" />
                </span>
              )}
            </div>
          );
        })}
      </div>
      <TestBreadcrumbsNotifications />
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

// TODO: Remove this when the notifications are implemented
const TestBreadcrumbsNotifications = () => {
  return (
    <div className="flex items-center">
      <div className="flex items-center gap-2 px-2">
        <div className="size-1.5 rounded bg-red-600" />
        <span>2</span>
      </div>

      <div className="flex items-center gap-2 px-2">
        <div className="size-1.5 rounded bg-blue-600" />
        <span>15</span>
      </div>
    </div>
  );
};
