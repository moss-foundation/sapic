import { useProjectsTrees } from "@/hooks/project";
import { Icon } from "@/lib/ui";

import { ActionMenu } from "..";
import { EntryIcon } from "../EntryIcon";
import BreadcrumbTree from "./BreadcrumbTree";
import { findNodeByIdInTree, findNodesSequence } from "./utils";

interface BreadcrumbsProps {
  collectionId?: string;
  nodeId?: string;
}

export const Breadcrumbs = ({ collectionId, nodeId }: BreadcrumbsProps) => {
  const { projectsTrees: collectionsTrees, isLoading } = useProjectsTrees();

  if (isLoading) {
    return null;
  }

  if (!collectionId || !nodeId) {
    return null;
  }

  if (!collectionsTrees || collectionsTrees.length === 0) {
    return null;
  }

  const activeTree = collectionsTrees?.find((tree) => tree.id === collectionId);
  if (!activeTree) {
    return null;
  }

  const activeNode = findNodeByIdInTree(activeTree, nodeId);
  if (!activeNode) {
    return null;
  }

  const nodesSequence = findNodesSequence(activeTree, activeNode);
  if (!nodesSequence) {
    return null;
  }

  return (
    <div className="flex items-center justify-between py-1">
      <div className="flex w-max items-center overflow-hidden text-(--moss-secondary-text) select-none">
        {nodesSequence.map((node, index) => {
          const lastItem = index === activeNode?.path.segments.length - 1;

          if (lastItem) {
            return (
              <div key={node.id} className="contents">
                <EntryIcon entry={node} />
                <span className="min-w-max">{node.name}</span>
              </div>
            );
          }

          return (
            <div key={node.id} className="contents">
              <ActionMenu.Root>
                <ActionMenu.Trigger className="flex min-w-max cursor-pointer items-center gap-1 px-1 py-0.5 hover:underline">
                  <EntryIcon entry={node} />
                  <span>{node.name} </span>
                </ActionMenu.Trigger>
                <ActionMenu.Content align="start">
                  <BreadcrumbTree tree={node} collectionId={collectionId} />
                </ActionMenu.Content>
              </ActionMenu.Root>
              {!lastItem && <Icon icon="ChevronRight" />}
            </div>
          );
        })}
      </div>
      <TestBreadcrumbsNotifications />
    </div>
  );
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
