import React from "react";

import { ActionButton, ActionMenu, Divider, IconLabelButton } from "@/components";
import { cn } from "@/utils";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";

import { collectionActionMenuItems } from "./HeadBarData";
import { getGitBranchMenuItems } from "./mockHeadBarData";
import NavigationButtons from "./NavigationButtons";

export interface HeadBarCenterItemsProps {
  isMedium: boolean;
  isXLarge: boolean;
  breakpoint: string;
  handleGitMenuAction: (action: string) => void;
  handleCollectionActionMenuAction: (action: string) => void;
  selectedBranch: string | null;
  collectionName: string;
  onRenameCollection: (newName: string) => void;
  onCollectionClick?: () => void;
  collectionButtonRef: React.RefObject<HTMLButtonElement>;
  os: string | null;
  onNavigateBack?: () => void;
  onNavigateForward?: () => void;
  canGoBack?: boolean;
  canGoForward?: boolean;
}

export const HeadBarCenterItems = ({
  isMedium,
  isXLarge,
  handleGitMenuAction,
  handleCollectionActionMenuAction,
  selectedBranch,
  collectionName,
  onRenameCollection,
  onCollectionClick,
  collectionButtonRef,
  os,
  onNavigateBack,
  onNavigateForward,
  canGoBack = true,
  canGoForward = true,
}: HeadBarCenterItemsProps) => {
  return (
    <div className="flex items-center gap-2" data-tauri-drag-region>
      <NavigationButtons
        onBack={onNavigateBack}
        onForward={onNavigateForward}
        canGoBack={canGoBack}
        canGoForward={canGoForward}
      />
      <div
        className={cn(
          "background-(--moss-headBar-primary-background) flex h-[26px] items-center rounded border border-[var(--moss-headBar-border-color)] px-0.5",
          isXLarge ? "" : os === "macos" ? "relative" : ""
        )}
        data-tauri-drag-region
      >
        <ActionButton
          icon="Refresh"
          iconClassName="text-(--moss-headBar-icon-primary-text)"
          customHoverBackground="hover:background-(--moss-headBar-primary-background-hover)"
          title="Reload"
        />
        <IconLabelButton
          ref={collectionButtonRef}
          leftIcon="UnloadedModule"
          leftIconClassName="text-(--moss-headBar-icon-primary-text)"
          className={
            isMedium
              ? "hover:background-(--moss-headBar-primary-background-hover) mr-[3px] h-[22px] w-[10vw]"
              : "hover:background-(--moss-headBar-primary-background-hover) mr-[30px] h-[22px] w-[10vw]"
          }
          title={collectionName}
          editable={true}
          onRename={onRenameCollection}
          onClick={onCollectionClick}
        />
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <ActionButton
              icon="MoreHorizontal"
              iconClassName="text-(--moss-headBar-icon-primary-text)"
              customHoverBackground="hover:background-(--moss-headBar-primary-background-hover)"
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
              className="hover:background-(--moss-headBar-primary-background-hover) ml-[-2px] h-[22px]"
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
    </div>
  );
};
