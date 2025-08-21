import { ReactNode, useEffect, useState } from "react";

import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { Icon, Icons } from "@/lib/ui";
import { cn } from "@/utils";

import { Divider } from "../Divider";

export interface PageHeaderProps {
  icon?: Icons;
  tabs?: ReactNode;
  toolbar?: ReactNode;
  className?: string;
  title?: string;
  props?: IDockviewPanelProps;
  onTitleChange?: (title: string) => void;
  disableTitleChange?: boolean;
  isRenamingTitle?: boolean;
  setIsRenamingTitle?: (isRenamingTitle: boolean) => void;
  handleRenamingFormCancel?: () => void;
}

export const PageHeader = ({
  icon,
  tabs,
  toolbar,
  className,
  props,
  onTitleChange,
  disableTitleChange = true,
  isRenamingTitle = false,
  setIsRenamingTitle,
  handleRenamingFormCancel,
  title: initialTitle,
}: PageHeaderProps) => {
  const [title, setTitle] = useState(initialTitle ?? "Untitled");

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

  const handleSubmit = () => {
    if (disableTitleChange) return;

    onTitleChange?.(title);
    setIsRenamingTitle?.(false);
  };

  const handleCancel = () => {
    setTitle(initialTitle ?? "Untitled");
    handleRenamingFormCancel?.();
  };

  return (
    <header
      className={cn("background-(--moss-primary-background) border-b border-(--moss-border-color) py-1.5", className)}
    >
      <div className="flex h-full items-center gap-3 px-3">
        <div className="flex min-w-0 items-center gap-1.5">
          {icon && <Icon icon={icon} className="size-[18px]" />}

          {isRenamingTitle ? (
            <form onSubmit={handleSubmit}>
              <input
                autoFocus
                value={title}
                onChange={(event) => setTitle(event.target.value)}
                onBlur={(event) => {
                  event.preventDefault();
                  handleSubmit();
                }}
                onKeyDown={(event) => {
                  if (event.key === "Enter") {
                    handleSubmit();
                  }
                  if (event.key === "Escape") {
                    handleCancel();
                  }
                }}
                className="field-sizing-content w-auto rounded text-[16px] leading-6 font-semibold text-(--moss-primary-text)"
              />
            </form>
          ) : (
            <h2
              onDoubleClick={() => setIsRenamingTitle?.(true)}
              className="truncate text-[16px] leading-6 font-semibold text-(--moss-primary-text)"
            >
              {title}
            </h2>
          )}
        </div>

        {tabs && (
          <>
            <Divider className="" />
            <div className="flex items-center">{tabs}</div>
          </>
        )}

        {toolbar && <div className="ml-auto flex items-center">{toolbar}</div>}
      </div>
    </header>
  );
};
