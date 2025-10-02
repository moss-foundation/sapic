import { Allotment, AllotmentHandle } from "allotment";
import { ComponentProps, forwardRef, useEffect, useState } from "react";

import { cn } from "@/utils";

const smoothHideClasses = `
  [&>.split-view-container>.split-view-view]:transition-all 
  [&>.split-view-container>.split-view-view]:duration-[0.15s] 
  [&>.split-view-container>.split-view-view]:ease-[ease-in-out] 
  [&>.split-view-container>.split-view-view]:will-change-[width,height] 
  [&.split-view-sash-dragging>.split-view-container>.split-view-view]:transition-none
`;

type ResizableProps = ComponentProps<typeof Allotment> & { smoothHide?: boolean };
export const Resizable = forwardRef<AllotmentHandle, ResizableProps>(
  ({ smoothHide = false, className, children, ...props }, ref) => {
    const [disableSmoothHide, setDisableSmoothHide] = useState(false);
    const [isReady, setIsReady] = useState(false);

    useEffect(() => {
      if (!smoothHide) return;

      const handleResize = () => {
        setDisableSmoothHide(true);
        setTimeout(() => {
          setDisableSmoothHide(false);
        }, 500);
      };

      window.addEventListener("resize", handleResize);

      return () => {
        window.removeEventListener("resize", handleResize);
      };
    }, [smoothHide]);

    // This is a workaround to deal with "Index out of bounds" error
    useEffect(() => {
      setTimeout(() => {
        setIsReady(true);
      }, 100);
    }, []);

    if (!isReady) return null;

    return (
      <Allotment
        ref={ref}
        className={cn({ [smoothHideClasses]: smoothHide && !disableSmoothHide }, className)}
        {...props}
      >
        {children}
      </Allotment>
    );
  }
);

export const ResizablePanel = Allotment.Pane;

export default { Resizable, ResizablePanel };
