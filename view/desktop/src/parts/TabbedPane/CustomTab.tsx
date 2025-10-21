import { IDockviewPanelHeaderProps } from "moss-tabs";
import { HTMLAttributes, MouseEvent, useCallback, useEffect, useState } from "react";

import { ResourceIcon } from "@/components/ResourceIcon";
import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";

export type CustomTabProps = IDockviewPanelHeaderProps &
  HTMLAttributes<HTMLDivElement> & {
    hideClose?: boolean;
    closeActionOverride?: () => void;
  };

export const CustomTab = ({
  api,
  containerApi: _containerApi,
  params,
  hideClose,
  closeActionOverride,
  onClick,
  tabLocation,
  ...props
}: CustomTabProps) => {
  const [title, setTitle] = useState(api.title || "");
  const [isCloseHovered, setIsCloseHovered] = useState(false);
  const [isActive, setIsActive] = useState(api.isActive);

  useEffect(() => {
    const disposable = api.onDidTitleChange?.((event) => {
      setTitle(event.title);
    });

    return () => {
      disposable?.dispose();
    };
  }, [api]);

  useEffect(() => {
    const disposable = api.onDidActiveChange?.((event) => {
      setIsActive(event.isActive);
    });

    return () => {
      disposable?.dispose();
    };
  }, [api]);

  const handleClose = useCallback(
    (event: MouseEvent<HTMLSpanElement>) => {
      event.preventDefault();

      if (closeActionOverride) {
        closeActionOverride();
      } else {
        api.close();
      }
    },
    [api, closeActionOverride]
  );

  const handleClick = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (event.defaultPrevented) {
        return;
      }

      api.setActive();

      onClick?.(event);
    },
    [api, onClick]
  );

  return (
    <div
      onClick={handleClick}
      className={cn(
        "group/customTab flex h-full items-center justify-center gap-1 px-3 hover:text-(--moss-primary-foreground)",
        {
          "border-b-1 border-(--moss-accent)": isActive,
          "border-b-1 border-(--moss-border)": !isActive,
        }
      )}
      tab-location={tabLocation}
      {...props}
    >
      <span
        className={cn("flex max-w-40 grow items-center gap-1", {
          "": isActive,
          "opacity-70 transition-opacity group-hover/customTab:opacity-100": !isActive,
        })}
      >
        {params?.iconType ? (
          <Icon icon={params?.iconType} className="size-4" />
        ) : params?.node ? (
          <div className="relative size-4 shrink-0">
            <ResourceIcon resource={params?.node} className="absolute top-0 right-0 size-4" />
          </div>
        ) : null}
        <span className="truncate">{title}</span>
      </span>

      {!hideClose && (
        <button
          className="flex items-center justify-center p-0"
          onPointerDown={(e) => e.preventDefault()}
          onClick={handleClose}
          onMouseEnter={() => setIsCloseHovered(true)}
          onMouseLeave={() => setIsCloseHovered(false)}
        >
          <Icon icon={isCloseHovered ? "CloseSmallHovered" : "CloseSmall"} />
        </button>
      )}
    </div>
  );
};

export default CustomTab;
