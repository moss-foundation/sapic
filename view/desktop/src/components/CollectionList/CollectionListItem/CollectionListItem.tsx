import { useState } from "react";

import { ActionButton, ActionMenu } from "@/components";
import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils";

export const CollectionListItem = ({ label }: { label: string }) => {
  const [showActionMenu, setShowActionMenu] = useState(false);

  return (
    <div className="group/CollectionListItem flex h-[26px] items-center justify-between pr-2 pl-4">
      <div className="flex items-center gap-2">
        <Icon icon="CollectionEnvironment" />
        <div>{label}</div>
        <div className="text-(--moss-secondary-text)">(15)</div>
      </div>

      <div
        className={cn("sr-only flex items-center gap-2 group-hover/CollectionListItem:not-sr-only", {
          "not-sr-only": showActionMenu,
        })}
      >
        <Icon icon="EnvironmentSelection" />
        <ActionMenu.Root onOpenChange={setShowActionMenu} modal={showActionMenu}>
          <ActionMenu.Trigger asChild>
            <ActionButton icon="MoreHorizontal" />
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            <ActionMenu.Item>Edit</ActionMenu.Item>
            <ActionMenu.Item>Delete</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Root>
      </div>
    </div>
  );
};
