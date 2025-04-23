import React from "react";

interface DividerProps {
  height?: "small" | "medium" | "large";
  className?: string;
}

export const Divider: React.FC<DividerProps> = ({ height = "medium", className = "" }) => {
  const heightClasses = {
    small: "h-[16px]",
    medium: "h-[18px]",
    large: "h-[20px]",
  };

  return (
    <div className={`mx-1 flex h-full items-center ${className}`}>
      <div className={`${heightClasses[height]} background-(--moss-divider-color) w-[1px]`}></div>
    </div>
  );
};
