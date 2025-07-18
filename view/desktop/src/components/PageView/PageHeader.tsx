import { cn } from "@/utils";
import React, { useEffect, useState } from "react";
import { Divider } from "../Divider";
import { PageHeaderProps } from "./types";

export const PageHeader: React.FC<PageHeaderProps> = ({ icon, tabs, toolbar, className, props }) => {
  const [title, setTitle] = useState("Untitled");

  useEffect(() => {
    const currentPanel = props?.containerApi?.getPanel(props.api.id);

    setTitle(currentPanel?.title ?? "Untitled");

    if (props?.api?.onDidTitleChange) {
      const disposable = props.api.onDidTitleChange((event) => {
        setTitle(event.title);
      });

      return () => {
        disposable?.dispose();
      };
    }
  }, [props?.api, props?.containerApi]);

  return (
    <header
      className={cn("background-(--moss-primary-background) h-9 border-b border-(--moss-border-color)", className)}
    >
      {/* Main Header Row - Title, Tabs, and Toolbar */}
      <div className="flex h-full items-center px-3">
        {/* Left side - Title and Icon */}
        <div className="flex min-w-0 flex-shrink-0 items-center gap-1.5">
          {icon && <div className="flex-shrink-0">{icon}</div>}
          <h1 className="truncate text-[16px] font-semibold text-(--moss-primary-text)">{title}</h1>
        </div>

        {/* Divider and Tabs - positioned after title */}
        {tabs && (
          <>
            <Divider className="mr-2 px-2" />
            <div className="-ml-1 flex items-center">{tabs}</div>
          </>
        )}

        {/*Right side - Toolbar */}
        {toolbar && <div className="ml-auto flex items-center">{toolbar}</div>}
      </div>
    </header>
  );
};
