import { useEffect, useRef, useState } from "react";

import { cn } from "@/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

export const TestDropTarget = () => {
  const ref = useRef<HTMLDivElement | null>(null);
  const label = "Drop target for element";

  const [dropAllowance, setDropAllowance] = useState<boolean | null>(null);

  useEffect(() => {
    const element = ref?.current;

    if (!element) return;

    return dropTargetForElements({
      element,

      onDragEnter({ self, source, location }) {
        // console.log("onDragEnter", { self, source, location });
        if (source.data.type === "Tab") {
          setDropAllowance(true);
        } else {
          setDropAllowance(false);
        }
      },
      onDrag({ self, source, location }) {
        console.log("onDrag", { self, source, location });
      },
      onDragLeave(args) {
        // console.log("onDragLeave", args);
        setDropAllowance(null);
      },
    });
  }, [label, ref]);

  return (
    <div
      ref={ref}
      className={cn("absolute inset-x-100 inset-y-40 bg-amber-300", {
        "bg-amber-500": dropAllowance === null,
        "bg-red-600": dropAllowance === false,
        "bg-green-600": dropAllowance === true,
      })}
    >
      {label}
    </div>
  );
};

export default TestDropTarget;
