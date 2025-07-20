import React from "react";

import { ActionButton } from "@/components";

export interface NavigationButtonsProps {
  onBack?: () => void;
  onForward?: () => void;
  canGoBack?: boolean;
  canGoForward?: boolean;
  className?: string;
}

export const NavigationButtons: React.FC<NavigationButtonsProps> = ({
  onBack,
  onForward,
  canGoBack = true,
  canGoForward = true,
  className = "",
}) => {
  return (
    <div className={`flex items-center gap-0 ${className}`}>
      <ActionButton
        icon="ChevronLeft"
        iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
        customHoverBackground="hover:background-(--moss-headBar-primary-background-hover)"
        title="Go back"
        onClick={onBack}
        disabled={!canGoBack}
        className={!canGoBack ? "cursor-not-allowed opacity-50" : ""}
      />
      <ActionButton
        icon="ChevronRight"
        iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
        customHoverBackground="hover:background-(--moss-headBar-primary-background-hover)"
        title="Go forward"
        onClick={onForward}
        disabled={!canGoForward}
        className={!canGoForward ? "cursor-not-allowed opacity-50" : ""}
      />
    </div>
  );
};

export default NavigationButtons;
