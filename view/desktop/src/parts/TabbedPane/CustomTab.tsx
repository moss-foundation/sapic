import React, { useCallback, useEffect, useState } from "react";

import { TreeNodeIcon } from "@/components/CollectionTree/TreeNode/TreeNodeIcon";
import { Icon } from "@/lib/ui/Icon";
import { IDockviewPanelHeaderProps } from "@repo/moss-tabs";

export type CustomTabProps = IDockviewPanelHeaderProps &
  React.HTMLAttributes<HTMLDivElement> & {
    hideClose?: boolean;
    closeActionOverride?: () => void;
  };

export const CustomTab: React.FC<CustomTabProps> = ({
  api,
  containerApi: _containerApi,
  params,
  hideClose,
  closeActionOverride,
  ...rest
}) => {
  // Get title from the API and subscribe to changes
  const [title, setTitle] = useState(api.title || "");
  const iconType = params?.iconType as string;
  const [isCloseHovered, setIsCloseHovered] = useState(false);

  // Subscribe to title changes
  useEffect(() => {
    setTitle(api.title || "");

    const disposable = api.onDidTitleChange?.((event) => {
      setTitle(event.title);
    });

    return () => {
      disposable?.dispose();
    };
  }, [api]);

  const onClose = useCallback(
    (event: React.MouseEvent<HTMLSpanElement>) => {
      event.preventDefault();

      if (closeActionOverride) {
        closeActionOverride();
      } else {
        api.close();
      }
    },
    [api, closeActionOverride]
  );

  const onPointerDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
  }, []);

  const onClick = useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (event.defaultPrevented) {
        return;
      }

      api.setActive();

      if (rest.onClick) {
        rest.onClick(event);
      }
    },
    [api, rest.onClick]
  );

  return (
    <div data-testid="dockview-custom-tab" {...rest} onClick={onClick} className="dv-default-tab">
      <span className="dv-default-tab-content flex max-w-40 items-center gap-1">
        {iconType && <TreeNodeIcon node={params?.node} isRootNode={false} />}
        <span className="truncate">{title}</span>
      </span>
      {!hideClose && (
        <div className="dv-default-tab-action" onPointerDown={onPointerDown} onClick={onClose}>
          <div onMouseEnter={() => setIsCloseHovered(true)} onMouseLeave={() => setIsCloseHovered(false)}>
            <Icon
              icon={isCloseHovered ? "CloseSmallHovered" : "CloseSmall"}
              className="text-[var(--moss-icon-primary-text)]"
            />
          </div>
        </div>
      )}
    </div>
  );
};

export default CustomTab;
