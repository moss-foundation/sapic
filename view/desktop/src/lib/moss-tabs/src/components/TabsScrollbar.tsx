import { OverlayScrollbarsComponent } from "overlayscrollbars-react";
import React from "react";

interface TabsScrollbarProps {
  children: React.ReactNode;
  className?: string;
}

export const TabsScrollbar: React.FC<TabsScrollbarProps> = ({ children, className }) => {
  return (
    <OverlayScrollbarsComponent
      options={{
        scrollbars: {
          autoHide: "move",
        },
        overflow: {
          x: "hidden", // Hide horizontal scrollbar
          y: "scroll", // Show vertical scrollbar
        },
      }}
      className={className}
      defer
    >
      {children}
    </OverlayScrollbarsComponent>
  );
};

export default TabsScrollbar;
