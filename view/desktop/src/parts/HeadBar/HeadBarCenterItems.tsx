import React from "react";

import { ActionButton, ActionMenu, Divider, IconLabelButton } from "@/components";
import { cn } from "@/utils";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";

import { collectionActionMenuItems } from "./HeadBarData";
import { getGitBranchMenuItems } from "./mockHeadBarData";

export interface HeadBarCenterItemsProps {
  isMedium: boolean;
  isXLarge: boolean;
  breakpoint: string;
  handleGitMenuAction: (action: string) => void;
  handleCollectionActionMenuAction: (action: string) => void;
  selectedBranch: string | null;
  collectionName: string;
  onRenameCollection: (newName: string) => void;
  collectionButtonRef: React.RefObject<HTMLButtonElement>;
  os: string | null;
}

export const HeadBarCenterItems = ({
  isMedium,
  isXLarge,
  handleGitMenuAction,
  handleCollectionActionMenuAction,
  selectedBranch,
  collectionName,
  onRenameCollection,
  collectionButtonRef,
  os,
}: HeadBarCenterItemsProps) => {
  return (
    <div
      className={cn(
        "flex h-[26px] items-center rounded border border-[var(--moss-headBar-border-color)] bg-[var(--moss-headBar-primary-background)] px-0.5",
        isXLarge ? "" : os === "macos" ? "relative" : ""
      )}
      data-tauri-drag-region
    >
      <IconLabelButton
        ref={collectionButtonRef}
        leftIcon="UnloadedModule"
        leftIconClassName="text-(--moss-headBar-icon-primary-text)"
        className={
          isMedium
            ? "mr-[3px] h-[22px] w-[10vw] hover:bg-[var(--moss-headBar-primary-background-hover)]"
            : "mr-[30px] h-[22px] w-[10vw] hover:bg-[var(--moss-headBar-primary-background-hover)]"
        }
        title={collectionName}
        editable={true}
        onRename={onRenameCollection}
      />
      <ActionButton
        icon="Refresh"
        iconClassName="text-(--moss-headBar-icon-primary-text)"
        customHoverBackground="hover:bg-[var(--moss-headBar-primary-background-hover)]"
        title="Reload"
      />
      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <ActionButton
            icon="MoreHorizontal"
            iconClassName="text-(--moss-headBar-icon-primary-text)"
            customHoverBackground="hover:bg-[var(--moss-headBar-primary-background-hover)]"
            className="mr-[-4px]"
            title="Collection Actions"
          />
        </ActionMenu.Trigger>
        <ActionMenu.Portal>
          <ActionMenu.Content>
            {collectionActionMenuItems.map((item) => renderActionMenuItem(item, handleCollectionActionMenuAction))}
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
      <Divider />
      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <IconLabelButton
            leftIcon="VCS"
            leftIconClassName="text-(--moss-headBar-icon-primary-text)"
            rightIcon="ChevronDown"
            className="ml-[-2px] h-[22px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
            title={selectedBranch || "main"}
            placeholder="No branch selected"
            showPlaceholder={!selectedBranch}
          />
        </ActionMenu.Trigger>
        <ActionMenu.Portal>
          <ActionMenu.Content>
            {getGitBranchMenuItems(selectedBranch).map((item) => renderActionMenuItem(item, handleGitMenuAction))}
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    </div>
  );
};
