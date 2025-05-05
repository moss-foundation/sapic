import React, { useCallback, useState } from "react";

import { Icon } from "@/components/Icon";
import { TestCollectionIcon } from "@/components/Tree/TestCollectionIcon";
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
  // Get title from the API
  const title = api.title || "";
  const iconType = params?.iconType as string;
  const [isCloseHovered, setIsCloseHovered] = useState(false);

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
      <span className="dv-default-tab-content flex items-center gap-1">
        {iconType && <TestCollectionIcon type={iconType} />}
        <span>{title}</span>
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
