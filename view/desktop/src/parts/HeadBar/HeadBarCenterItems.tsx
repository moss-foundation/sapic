import React from "react";

import { ActionButton, ActionMenu, Divider, IconLabelButton } from "@/components";
import { cn } from "@/utils";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";

import { collectionActionMenuItems } from "./HeadBarData";
import { getGitBranchMenuItems } from "./mockHeadBarData";
import NavigationButtons from "./NavigationButtons";
import ZoomButtons from "./ZoomButtons";
import RequestPath from "./RequestPath";

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
  onZoomIn?: () => void;
  onZoomOut?: () => void;
  canZoomIn?: boolean;
  canZoomOut?: boolean;
  currentZoom?: number;
}

export const HeadBarCenterItems = ({
  isXLarge,
  breakpoint,
  handleGitMenuAction,
  handleCollectionActionMenuAction,
  selectedBranch,
  os,
  onNavigateBack,
  onNavigateForward,
  canGoBack = true,
  canGoForward = true,
  onZoomIn,
  onZoomOut,
  canZoomIn = true,
  canZoomOut = true,
  currentZoom = 100,
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
          "background-(--moss-headBar-primary-background) flex h-7 items-center rounded border border-[var(--moss-headBar-border-color)] px-2",
          {
            "min-w-80": breakpoint === "sm",
            "min-w-96": breakpoint === "md",
            "min-w-[28rem]": breakpoint === "lg",
            "min-w-[32rem]": breakpoint === "xl" || breakpoint === "2xl",
          },
          isXLarge ? "" : os === "macos" ? "relative" : ""
        )}
        data-tauri-drag-region
      >
        <ActionButton
          icon="Refresh"
          iconClassName="text-(--moss-headBar-icon-primary-text)"
          customHoverBackground="hover:background-(--moss-headBar-primary-background-hover)"
          className="mx-1"
          title="Reload"
        />
        <RequestPath
          className={cn("min-w-0 text-(--moss-headBar-icon-primary-text)", {
            "max-w-48": breakpoint === "sm",
            "max-w-60": breakpoint === "md",
            "max-w-72": breakpoint === "lg",
            "max-w-80": breakpoint === "xl" || breakpoint === "2xl",
          })}
        />
        <div className="flex-1" />
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <ActionButton
              icon="MoreHorizontal"
              iconClassName="text-(--moss-headBar-icon-primary-text)"
              customHoverBackground="hover:background-(--moss-headBar-primary-background-hover)"
              className="-mr-1"
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
              className="hover:background-(--moss-headBar-primary-background-hover) -ml-0.5 h-6"
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
      <ZoomButtons
        onZoomIn={onZoomIn}
        onZoomOut={onZoomOut}
        canZoomIn={canZoomIn}
        canZoomOut={canZoomOut}
        currentZoom={currentZoom}
      />
    </div>
  );
};
