import { HTMLAttributes, MouseEvent, useCallback, useEffect, useState } from "react";

import { EntryIcon } from "@/components/EntryIcon";
import { Icon } from "@/lib/ui/Icon";
import { IDockviewPanelHeaderProps } from "@repo/moss-tabs";

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
  ...props
}: CustomTabProps) => {
  const [title, setTitle] = useState(api.title || "");
  const [isCloseHovered, setIsCloseHovered] = useState(false);

  useEffect(() => {
    const disposable = api.onDidTitleChange?.((event) => {
      setTitle(event.title);
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
      data-testid="dockview-custom-tab"
      className="group/customTab flex h-full items-center justify-center gap-1 px-3 hover:text-(--moss-primary-text)"
      {...props}
    >
      <span className="flex max-w-40 grow items-center gap-1 opacity-70 transition-opacity group-hover/customTab:opacity-100">
        {params?.iconType ? (
          <Icon icon={params?.iconType} className="size-4" />
        ) : params?.node ? (
          ~(
            <div className="relative size-4 shrink-0">
              <EntryIcon entry={params?.node} className="absolute top-0 right-0" />
            </div>
          )
        ) : null}
        <span className="truncate">{title}</span>
      </span>

      {!hideClose && (
        <button
          className="flex items-center justify-center p-0"
          onPointerDown={(e) => e.preventDefault()}
          onClick={handleClose}
        >
          <div onMouseEnter={() => setIsCloseHovered(true)} onMouseLeave={() => setIsCloseHovered(false)}>
            <Icon
              icon={isCloseHovered ? "CloseSmallHovered" : "CloseSmall"}
              className="text-(--moss-icon-primary-text)"
            />
          </div>
        </button>
      )}
    </div>
  );
};

export default CustomTab;
