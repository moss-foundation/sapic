import React from "react";

import { ActionButton } from "@/components";

export interface ZoomButtonsProps {
  onZoomIn?: () => void;
  onZoomOut?: () => void;
  canZoomIn?: boolean;
  canZoomOut?: boolean;
  currentZoom?: number;
  className?: string;
}

export const ZoomButtons: React.FC<ZoomButtonsProps> = ({
  onZoomIn,
  onZoomOut,
  canZoomIn = true,
  canZoomOut = true,
  currentZoom = 100,
  className = "",
}) => {
  return (
    <div className={`flex items-center gap-0 ${className}`}>
      <ActionButton
        icon="Minus"
        iconClassName="text-(--moss-statusBarItem-foreground) size-4.5"
        title="Zoom out"
        onClick={onZoomOut}
        disabled={!canZoomOut}
        className={!canZoomOut ? "cursor-not-allowed opacity-50" : ""}
      />
      <span className="text-(--moss-statusBarItem-foreground) px-1 text-sm">{currentZoom}%</span>
      <ActionButton
        icon="Add"
        iconClassName="text-(--moss-statusBarItem-foreground) size-4.5"
        title="Zoom in"
        onClick={onZoomIn}
        disabled={!canZoomIn}
        className={!canZoomIn ? "cursor-not-allowed opacity-50" : ""}
      />
    </div>
  );
};

export default ZoomButtons;
