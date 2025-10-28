import { useProjectsTrees } from "@/hooks/project";
import { Icon } from "@/lib/ui";

import { ActionMenu } from "..";
import { ResourceIcon } from "../ResourceIcon";
import BreadcrumbTree from "./BreadcrumbTree";
import { findNodeByIdInTree, findNodesSequence } from "./utils";

interface BreadcrumbsProps {
  projectId?: string;
  nodeId?: string;
}

export const Breadcrumbs = ({ projectId, nodeId }: BreadcrumbsProps) => {
  const { projectsTrees: projectsTrees, isLoading } = useProjectsTrees();

  if (isLoading) {
    return null;
  }

  if (!projectId || !nodeId) {
    return null;
  }

  if (!projectsTrees || projectsTrees.length === 0) {
    return null;
  }

  const activeTree = projectsTrees?.find((tree) => tree.id === projectId);
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
      <div className="text-(--moss-secondary-foreground) flex w-max select-none items-center overflow-hidden">
        {nodesSequence.map((node, index) => {
          const lastItem = index === activeNode?.path.segments.length - 1;

          if (lastItem) {
            return (
              <div key={node.id} className="contents">
                <ResourceIcon resource={node} />
                <span className="min-w-max">{node.name}</span>
              </div>
            );
          }

          return (
            <div key={node.id} className="contents">
              <ActionMenu.Root>
                <ActionMenu.Trigger className="flex min-w-max cursor-pointer items-center gap-1 px-1 py-0.5 hover:underline">
                  <ResourceIcon resource={node} />
                  <span>{node.name} </span>
                </ActionMenu.Trigger>
                <ActionMenu.Content align="start">
                  <BreadcrumbTree tree={node} projectId={projectId} />
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
