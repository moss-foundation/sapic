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
          autoHide: "scroll",
          theme: "os-theme-dark",
          dragScroll: true,
          clickScroll: true,
        },
        overflow: {
          x: "scroll", // Show horizontal scrollbar
          y: "hidden", // Hide vertical scrollbar
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
