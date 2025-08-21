import { ReactNode, useEffect, useState } from "react";

import { useRenameEntryForm } from "@/hooks/useRenameEntryForm";
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
}

export const PageHeader = ({ icon, tabs, toolbar, className, props }: PageHeaderProps) => {
  const [title, setTitle] = useState("Untitled");

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useRenameEntryForm(
    props?.params?.node,
    props?.params?.collectionId
  );

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

  const handleRename = () => {
    setIsRenamingNode(true);
  };

  return (
    <header
      className={cn("background-(--moss-primary-background) border-b border-(--moss-border-color) py-1.5", className)}
    >
      <div className="flex h-full items-center gap-3 px-3">
        <div className="flex min-w-0 items-center gap-1.5">
          {icon && <Icon icon={icon} className="size-[18px]" />}

          {isRenamingNode ? (
            <form
              onSubmit={(event) => {
                event.preventDefault();
                handleRenamingFormSubmit(title);
              }}
            >
              <input
                autoFocus
                value={title}
                onChange={(event) => setTitle(event.target.value)}
                onBlur={(event) => {
                  event.preventDefault();
                  handleRenamingFormSubmit(title);
                }}
                onKeyDown={(event) => {
                  if (event.key === "Enter") {
                    handleRenamingFormSubmit(title);
                  }
                  if (event.key === "Escape") {
                    handleRenamingFormCancel();
                  }
                }}
                className="field-sizing-content w-auto rounded text-[16px] leading-6 font-semibold text-(--moss-primary-text)"
              />
            </form>
          ) : (
            <h2
              onDoubleClick={handleRename}
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
