import { ReactNode, useEffect, useState } from "react";

import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { Icon, Icons } from "@/lib/ui";
import { cn } from "@/utils";

import { Divider } from "../Divider";
import InputPlain from "../InputPlain";

export interface PageHeaderProps extends IDockviewPanelProps {
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
  onTitleChange,
  disableTitleChange = true,
  isRenamingTitle = false,
  setIsRenamingTitle,
  handleRenamingFormCancel,
  title: initialTitle,
  api,
}: PageHeaderProps) => {
  const [title, setTitle] = useState(initialTitle);

  useEffect(() => {
    if (initialTitle) {
      setTitle(initialTitle);
      api?.setTitle(initialTitle);
    }
  }, [initialTitle, api]);

  const handleSubmit = () => {
    if (disableTitleChange || title === initialTitle) {
      setIsRenamingTitle?.(false);
      return;
    }

    if (!title || title === "") {
      setTitle(initialTitle ?? "Untitled");
      setIsRenamingTitle?.(false);
      return;
    }

    onTitleChange?.(title);
    setIsRenamingTitle?.(false);
  };

  const handleCancel = () => {
    setTitle(initialTitle ?? "Untitled");
    handleRenamingFormCancel?.();
  };

  const handleStartRenaming = () => {
    if (disableTitleChange) {
      return;
    }

    setIsRenamingTitle?.(true);
  };

  return (
    <header className={cn("background-(--moss-primary-background) py-1.5", className)}>
      <div className="flex h-full items-center gap-3 px-5">
        <div className="flex min-w-0 grow items-center justify-start gap-1.5">
          {icon && <Icon icon={icon} className="size-[18px]" />}
          {isRenamingTitle ? (
            <form
              onSubmit={(event) => {
                event.preventDefault();
                handleSubmit();
              }}
              className="-mx-1 w-full max-w-[200px] px-1"
            >
              <InputPlain
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
                className="w-full rounded-xs py-0 text-[16px] leading-6 font-semibold text-(--moss-primary-text) has-[input:focus-within]:outline-offset-1"
                inputFieldClassName="-mx-2"
              />
            </form>
          ) : (
            <button
              onClick={handleStartRenaming}
              className={cn(
                "-mx-1 truncate overflow-hidden rounded px-1 text-left text-[16px] leading-6 font-semibold text-(--moss-primary-text)",
                {
                  "hover:background-(--moss-secondary-background-hover) w-full max-w-[200px] cursor-text":
                    !disableTitleChange,
                }
              )}
            >
              <h2 className="truncate">{title}</h2>
            </button>
          )}
          {tabs && (
            <>
              <Divider className="" />
              <div className="flex items-center">{tabs}</div>
            </>
          )}
        </div>

        {toolbar && <div className="ml-auto flex items-center">{toolbar}</div>}
      </div>
    </header>
  );
};
