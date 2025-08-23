import React from "react";

import { cn } from "@/utils";

interface DividerProps {
  height?: "small" | "medium" | "large";
  className?: string;
}

export const Divider: React.FC<DividerProps> = ({ height = "medium", className = "" }) => {
  const heightClasses = {
    small: "h-4",
    medium: "h-4.5",
    large: "h-5",
  };

  return (
    <div className={cn("flex h-full items-center", className)}>
      <div className={`${heightClasses[height]} background-(--moss-divider-color) w-[1px]`}></div>
    </div>
  );
};
