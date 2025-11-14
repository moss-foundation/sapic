import React from "react";

import { ActionButton } from "@/workbench/ui/components";

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
        icon="ArrowLeft"
        iconClassName="size-4.5"
        title="Go back"
        onClick={onBack}
        disabled={!canGoBack}
        className={!canGoBack ? "cursor-not-allowed opacity-50" : ""}
      />
      <ActionButton
        icon="ArrowRight"
        iconClassName="size-4.5"
        title="Go forward"
        onClick={onForward}
        disabled={!canGoForward}
        className={!canGoForward ? "cursor-not-allowed opacity-50" : ""}
      />
    </div>
  );
};

export default NavigationButtons;
