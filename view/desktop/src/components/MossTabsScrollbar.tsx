import React from "react";

import { Scrollbar } from "./Scrollbar";

interface MossTabsScrollbarProps {
  children: React.ReactNode;
  className?: string;
}

export const MossTabsScrollbar: React.FC<MossTabsScrollbarProps> = ({ children, className }) => {
  return (
    <Scrollbar
      options={{
        scrollbars: {
          autoHide: "move",
          theme: "os-theme-dark",
        },
        overflow: {
          x: "hidden", // Hide horizontal scrollbar
          y: "scroll", // Show vertical scrollbar
        },
      }}
      className={className}
    >
      {children}
    </Scrollbar>
  );
};

export default MossTabsScrollbar;
