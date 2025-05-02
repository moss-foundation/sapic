import React from "react";
import { ActionButton, Divider, IconLabelButton } from "@/components";
import { cn } from "@/utils";
import ActionMenu from "@/components/ActionMenu/ActionMenu";
import { getGitBranchMenuItems } from "./mockHeadBarData";
import { collectionActionMenuItems } from "./HeadBarData";

export interface HeadBarCenterItemsProps {
  isMedium: boolean;
  isXLarge: boolean;
  breakpoint: string;
  gitMenuOpen: boolean;
  setGitMenuOpen: (open: boolean) => void;
  handleGitMenuAction: (action: string) => void;
  collectionActionMenuOpen: boolean;
  setCollectionActionMenuOpen: (open: boolean) => void;
  handleCollectionActionMenuAction: (action: string) => void;
  selectedBranch: string | null;
  collectionName: string;
  onRenameCollection: (newName: string) => void;
  collectionButtonRef: React.RefObject<HTMLButtonElement>;
}

export const HeadBarCenterItems = ({
  isMedium,
  isXLarge,
  gitMenuOpen,
  setGitMenuOpen,
  handleGitMenuAction,
  collectionActionMenuOpen,
  setCollectionActionMenuOpen,
  handleCollectionActionMenuAction,
  selectedBranch,
  collectionName,
  onRenameCollection,
  collectionButtonRef,
}: HeadBarCenterItemsProps) => {
  return (
    <div
      className={cn(
        "flex h-[26px] items-center rounded border border-[var(--moss-headBar-border-color)] bg-[var(--moss-headBar-primary-background)] px-0.5",
        isXLarge ? "" : "absolute left-1/2 -translate-x-1/2 transform"
      )}
      data-tauri-drag-region
    >
      <IconLabelButton
        ref={collectionButtonRef}
        leftIcon="HeadBarCollection"
        leftIconClassName="text-(--moss-headBar-icon-primary-text)"
        className={
          isMedium
            ? "mr-[3px] h-[22px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
            : "mr-[30px] h-[22px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
        }
        title={collectionName}
        editable={true}
        onRename={onRenameCollection}
      />
      <ActionButton
        icon="Reload"
        iconClassName="text-(--moss-headBar-icon-primary-text)"
        customHoverBackground="hover:bg-[var(--moss-headBar-primary-background-hover)]"
        title="Reload"
      />
      <ActionMenu
        items={collectionActionMenuItems}
        trigger={
          <ActionButton
            icon="ThreeVerticalDots"
            iconClassName="text-(--moss-headBar-icon-primary-text)"
            customHoverBackground="hover:bg-[var(--moss-headBar-primary-background-hover)]"
            className="mr-[-4px]"
            title="Collection Actions"
          />
        }
        open={collectionActionMenuOpen}
        onOpenChange={setCollectionActionMenuOpen}
        onSelect={(item) => {
          handleCollectionActionMenuAction(item.id);
        }}
      />
      <Divider />
      <ActionMenu
        items={getGitBranchMenuItems(selectedBranch)}
        trigger={
          <IconLabelButton
            leftIcon="HeadBarGit"
            leftIconClassName="text-(--moss-headBar-icon-primary-text)"
            rightIcon="ChevronDown"
            className="ml-[-2px] h-[22px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
            title={selectedBranch || "main"}
            placeholder="No branch selected"
            showPlaceholder={!selectedBranch}
          />
        }
        open={gitMenuOpen}
        onOpenChange={setGitMenuOpen}
        onSelect={(item) => {
          handleGitMenuAction(item.id);
        }}
      />
    </div>
  );
};
