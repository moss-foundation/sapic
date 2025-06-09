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
          "cursor-pointer rounded px-2 py-0.5 text-base transition-colors",
          isActive
            ? "background-(--moss-info-background) text-(--moss-primary)"
            : "background-(--moss-primary-background) hover:background-(--moss-icon-secondary-background-hover) text-(--moss-primary-text)"
        ),
      });
    }
    return child;
  });

  return <div className={cn("flex items-center gap-0.5", className)}>{styledChildren}</div>;
};
