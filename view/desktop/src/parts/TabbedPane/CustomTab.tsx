import { HTMLAttributes, MouseEvent, useCallback, useEffect, useState } from "react";

import { TreeNodeIcon } from "@/components/CollectionTree/TreeNode/TreeNodeIcon";
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
  ...rest
}: CustomTabProps) => {
  const [title, setTitle] = useState(api.title || "");
  const [isCloseHovered, setIsCloseHovered] = useState(false);

  // Subscribe to title changes
  // TODO: In theory, in the future, tab's title should be handled by panel itself
  useEffect(() => {
    setTitle(api.title || "");

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
    <div {...rest} onClick={handleClick} data-testid="dockview-custom-tab" className="dv-default-tab">
      <span className="dv-default-tab-content flex max-w-40 items-center gap-1">
        {params?.node && <TreeNodeIcon node={params?.node} />}
        <span className="truncate">{title}</span>
      </span>

      {!hideClose && (
        <button className="dv-default-tab-action cursor-pointer" onClick={handleClose}>
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
