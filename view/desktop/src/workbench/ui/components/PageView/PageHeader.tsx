import { IDockviewPanelProps } from "moss-tabs";
import { ReactNode, useEffect, useState } from "react";

import { Icon, Icons } from "@/lib/ui";
import Input from "@/lib/ui/Input";
import { cn } from "@/utils";

import { Divider } from "../Divider";

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
              <Input
                intent="plain"
                autoFocus
                value={title}
                onChange={(event) => setTitle(event.target.value)}
                onBlur={(event) => {
                  event.preventDefault();
                  handleSubmit();
                }}
                onKeyDown={(event) => {
                  if (event.key === "Enter") handleSubmit();
                  if (event.key === "Escape") handleCancel();
                }}
                className="rounded-xs text-(--moss-primary-foreground) w-full py-0 text-[16px] font-semibold leading-6 has-[input:focus-within]:outline-offset-1"
                inputFieldClassName="-mx-2"
              />
            </form>
          ) : (
            <button
              onClick={handleStartRenaming}
              className={cn(
                "text-(--moss-primary-foreground) -mx-1 overflow-hidden truncate rounded px-1 text-left text-[16px] font-semibold leading-6",
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
              <Divider />
              <div className="flex items-center">{tabs}</div>
            </>
          )}
        </div>

        {toolbar && <div className="ml-auto flex items-center">{toolbar}</div>}
      </div>
    </header>
  );
};
