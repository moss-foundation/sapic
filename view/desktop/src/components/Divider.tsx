import React from "react";

interface DividerProps {
  height?: string;
  className?: string;
}

export const Divider: React.FC<DividerProps> = ({ height = "18px", className = "" }) => {
  return (
    <div className={`mx-1 flex h-full items-center ${className}`}>
      <div className={`h-[${height}] background-(--moss-divider-color) w-[1px]`}></div>
    </div>
  );
};
