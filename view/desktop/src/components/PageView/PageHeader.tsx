import React from "react";
import { cn } from "@/utils";
import { PageHeaderProps } from "./types";

export const PageHeader: React.FC<PageHeaderProps> = ({ title, icon, tabs, toolbar, className }) => {
  return (
    <header
      className={cn("h-8 border-b border-gray-200 bg-gray-50 dark:border-stone-800 dark:bg-stone-900", className)}
    >
      {/* Main Header Row - Title, Tabs, and Toolbar */}
      <div className="flex h-full items-center px-3">
        {/* Left side - Title and Icon */}
        <div className="flex min-w-0 flex-shrink-0 items-center gap-2">
          {icon && <div className="flex-shrink-0">{icon}</div>}
          <h1 className="truncate text-sm font-medium text-gray-900 dark:text-white">{title}</h1>
        </div>

        {/* Tabs - positioned after title */}
        {tabs && <div className="ml-6 flex items-center">{tabs}</div>}

        {/* Toolbar - pushed to the far right */}
        {toolbar && <div className="ml-auto flex items-center">{toolbar}</div>}
      </div>
    </header>
  );
};
