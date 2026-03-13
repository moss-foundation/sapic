import { RefObject, useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { usePutResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/usePutResourcesListItemState";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ResourcesTreeActions } from "./ResourcesTreeActions";

interface ResourcesTreeHeaderProps {
  ref: RefObject<HTMLHeadingElement | null>;
  expanded: boolean;
  offsetLeft: number;
  offsetRight: number;
  setIsAddingFileNode: () => void;
  setIsAddingFolderNode: () => void;
}

export const ResourcesTreeHeader = ({
  ref,
  expanded,
  offsetLeft,
  offsetRight,
  setIsAddingFileNode,
  setIsAddingFolderNode,
}: ResourcesTreeHeaderProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { id } = useContext(ProjectTreeContext);

  const { mutate: updateResourcesListState } = usePutResourcesListItemState();

  const onHeaderClick = () => {
    if (expanded) return;

    updateResourcesListState({
      resourcesListItemId: id,
      expanded: !expanded,
      workspaceId: currentWorkspaceId,
    });
  };

  const onIconClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    updateResourcesListState({
      resourcesListItemId: id,
      expanded: !expanded,
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.ListHeader paddingLeft={offsetLeft} paddingRight={offsetRight} ref={ref}>
      <Tree.ListHeaderDetails className="cursor-pointer text-sm" onClick={onHeaderClick}>
        <button onClick={onIconClick} className="flex cursor-pointer items-center justify-center rounded-full">
          <Icon icon="ChevronRight" className={cn(expanded && "rotate-90")} />
        </button>

        <Tree.ListLabel label="Resources" />
        <Tree.ListDirCount count={-1} />
      </Tree.ListHeaderDetails>

      <ResourcesTreeActions setIsAddingFileNode={setIsAddingFileNode} setIsAddingFolderNode={setIsAddingFolderNode} />
    </Tree.ListHeader>
  );
};
