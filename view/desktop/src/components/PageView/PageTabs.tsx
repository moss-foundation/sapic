import React from "react";
import { cn } from "@/utils";
import { PageTabsProps } from "./types";

export const PageTabs: React.FC<PageTabsProps> = ({ children, className }) => {
  // Apply tab button styling to all button children
  const styledChildren = React.Children.map(children, (child) => {
    if (React.isValidElement(child) && child.type === "button") {
      const isActive = child.props["data-active"] === true;
      return React.cloneElement(child as React.ReactElement<any>, {
        className: cn(
          "rounded px-2 py-0.5 text-base transition-colors",
          isActive
            ? "background-(--moss-blue-12) text-(--moss-blue-4)"
            : "bg-white text-(--moss-gray-1) hover:bg-gray-100 dark:bg-stone-800 dark:text-gray-300 dark:hover:bg-stone-700"
        ),
      });
    }
    return child;
  });

  return <div className={cn("flex items-center gap-0.5", className)}>{styledChildren}</div>;
};
