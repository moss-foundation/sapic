import React, { useCallback } from "react";
import { IDockviewPanelHeaderProps } from "@repo/moss-tabs";
import { TestCollectionIcon } from "@/components/Tree/TestCollectionIcon";

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
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 4L4 12" stroke="#6C707E" strokeWidth="1.33333" strokeLinecap="round" strokeLinejoin="round" />
            <path d="M4 4L12 12" stroke="#6C707E" strokeWidth="1.33333" strokeLinecap="round" strokeLinejoin="round" />
          </svg>
        </div>
      )}
    </div>
  );
};

export default CustomTab;
