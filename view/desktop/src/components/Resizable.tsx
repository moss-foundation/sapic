import { Allotment, AllotmentHandle } from "allotment";

import { cn } from "@/utils";

import "allotment/dist/style.css";

import { ComponentProps, forwardRef } from "react";

const smoothHideClasses =
  "[&>.split-view-container>.split-view-view]:transition-all [&>.split-view-container>.split-view-view]:duration-[0.15s] [&>.split-view-container>.split-view-view]:ease-[ease-in-out] [&>.split-view-container>.split-view-view]:will-change-[width,height] [&.split-view-sash-dragging>.split-view-container>.split-view-view]:transition-none";

type ResizableProps = ComponentProps<typeof Allotment> & { smoothHide?: boolean };
export const Resizable = forwardRef<AllotmentHandle, ResizableProps>(
  ({ smoothHide = false, className, children, ...props }, ref) => {
    return (
      <Allotment ref={ref} className={cn({ [smoothHideClasses]: smoothHide }, className)} {...props}>
        {children}
      </Allotment>
    );
  }
);

export const ResizablePanel = Allotment.Pane;

export default { Resizable, ResizablePanel };
