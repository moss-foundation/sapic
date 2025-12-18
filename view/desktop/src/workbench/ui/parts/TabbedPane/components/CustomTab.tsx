import { IDockviewPanelHeaderProps } from "moss-tabs";
import { HTMLAttributes, MouseEvent, useCallback, useEffect, useEffectEvent, useState } from "react";

import { Icon, Icons } from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { ResourceIcon } from "@/workbench/ui/components/ResourceIcon";

export type CustomTabProps = IDockviewPanelHeaderProps &
  HTMLAttributes<HTMLDivElement> & {
    hideClose?: boolean;
    closeActionOverride?: () => void;
    tabIcon?: Icons;
  };

export const CustomTab = ({
  api,
  containerApi: _containerApi,
  params,
  hideClose,
  onClick,
  tabLocation,
  ...props
}: CustomTabProps) => {
  const [title, setTitle] = useState(api.title || "");
  const [isCloseHovered, setIsCloseHovered] = useState(false);
  const [isActive, setIsActive] = useState(api.isActive);

  const setupListeners = useEffectEvent(() => {
    const titleListener = api.onDidTitleChange?.((event) => {
      setTitle(event.title);
    });

    const activePanelListener = api.onDidActiveChange?.((event) => {
      setIsActive(event.isActive);
    });

    return () => {
      titleListener?.dispose();
      activePanelListener?.dispose();
    };
  });

  useEffect(() => setupListeners(), [api]);

  const handleClose = useCallback(
    (event: MouseEvent<HTMLSpanElement>) => {
      event.preventDefault();

      api.close();
    },
    [api]
  );

  const handleClick = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (event.defaultPrevented) return;

      api.setActive();
      onClick?.(event);
    },
    [api, onClick]
  );

  return (
    <div
      onClick={handleClick}
      className={cn(
        "group/customTab hover:text-(--moss-primary-foreground) flex h-full items-center justify-center gap-1 px-3",
        {
          "border-b-1 border-(--moss-accent)": isActive,
          "border-b-1 border-(--moss-border)": !isActive,
        }
      )}
      //react React does not recognize the `tabLocation` prop on a DOM element, we have to use the `tab-location` attribute
      tab-location={tabLocation}
      {...props}
    >
      <span
        className={cn("flex max-w-40 grow items-center gap-1", {
          "": isActive,
          "opacity-70 transition-opacity group-hover/customTab:opacity-100": !isActive,
        })}
      >
        {params?.tabIcon ? (
          <Icon icon={params?.tabIcon} className="size-4" />
        ) : params?.node ? (
          <div className="relative size-4 shrink-0">
            <ResourceIcon resource={params?.node} className="absolute right-0 top-0 size-4" />
          </div>
        ) : params?.tabIcon ? (
          <Icon icon={params?.tabIcon} className="size-4" />
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
