import { useState } from "react";

import { ActionButton, ActionMenu } from "@/components";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface GroupedEnvironmentsListItemProps {
  environment: StreamEnvironmentsEvent;
}

export const GroupedEnvironmentsListItem = ({ environment }: GroupedEnvironmentsListItemProps) => {
  const [showActionMenu, setShowActionMenu] = useState(false);
  const { activePanelId, addOrFocusPanel } = useTabbedPaneStore();

  const onClick = () => {
    addOrFocusPanel({
      id: `GroupedEnvironmentsListItem-${environment.id}`,
      component: "Default",
      title: environment.name,
      params: {
        iconType: "GroupedEnvironment",
      },
    });
  };

  const isActive = activePanelId === `GroupedEnvironmentsListItem-${environment.id}`;

  return (
    <Tree.Node isChildDropBlocked={false} instruction={null} onClick={onClick}>
      <Tree.NodeControls
        hideDragHandle
        depth={1}
        isChildDropBlocked={false}
        isActive={isActive}
        isRootNode={false}
        instruction={null}
        isLastChild={false}
      >
        <Tree.NodeTriggers>
          <Icon icon="GroupedEnvironment" />
          <div className="truncate">{environment.name}</div>
          <div className="text-(--moss-secondary-text)">(15)</div>
        </Tree.NodeTriggers>

        <Tree.NodeActions>
          <Tree.ActionsHover forceVisible={showActionMenu}>
            <Icon icon="EnvironmentSelection" />
            <ActionMenu.Root onOpenChange={setShowActionMenu} modal={showActionMenu}>
              <ActionMenu.Trigger asChild>
                <ActionButton icon="MoreHorizontal" customHoverBackground="hover:background-(--moss-gray-10)" />
              </ActionMenu.Trigger>
              <ActionMenu.Portal>
                <ActionMenu.Content>
                  <ActionMenu.Item>Edit</ActionMenu.Item>
                  <ActionMenu.Item>Delete</ActionMenu.Item>
                </ActionMenu.Content>
              </ActionMenu.Portal>
            </ActionMenu.Root>
          </Tree.ActionsHover>
        </Tree.NodeActions>
      </Tree.NodeControls>
    </Tree.Node>
  );
};
