import { useState } from "react";

import { cn } from "@/utils";

export interface SidebarEdgeHandlerProps {
  alignment?: "left" | "right" | "bottom";
  onClick?: () => void;
}

export const SidebarEdgeHandler = ({ alignment, onClick }: SidebarEdgeHandlerProps) => {
  const [showBg, setShowBg] = useState(false);
  return (
    <div
      className={cn("absolute z-40", {
        "left-0 h-full w-2": alignment === "left",
        "right-0 h-full w-2": alignment === "right",
        "bottom-0 h-2 w-full": alignment === "bottom",
      })}
    >
      {/* handle bg*/}
      <div
        className={cn(`background-(--moss-accent)/50 absolute z-40 hidden cursor-pointer`, {
          "left-0 top-0 h-full w-3": alignment === "left",
          "right-0 top-0 h-full w-3": alignment === "right",
          "bottom-0 left-0 h-3 w-full": alignment === "bottom",
          "block": showBg,
        })}
        onMouseEnter={() => setShowBg(true)}
        onMouseLeave={() => setShowBg(false)}
        onClick={onClick}
      />

      {/* handle */}
      <div
        className={cn(
          `background-(--moss-accent)/50 hover:background-(--moss-accent)/80 absolute z-50 cursor-pointer rounded`,
          {
            "inset-y-[calc(50%-64px)] left-[3px] h-32 w-1.5": alignment === "left",
            "inset-y-[calc(50%-64px)] right-[3px] h-32 w-1.5": alignment === "right",
            "inset-x-[calc(50%-64px)] bottom-[3px] h-1.5 w-32": alignment === "bottom",
            "background-(--moss-accent)/80": showBg,
          }
        )}
        onMouseEnter={() => setShowBg(true)}
        onClick={onClick}
      />
    </div>
  );
};
