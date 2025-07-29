import React, { useEffect, useRef, useState } from "react";

import { Item as ToggleGroupItem, Root as ToggleGroupRoot } from "@/components/ToggleGroup";
import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";

type ToggleValue = "request" | "design";

interface ModeToggleProps {
  className?: string;
  compact?: boolean;
}

export const ModeToggle: React.FC<ModeToggleProps> = ({ className, compact = false }) => {
  const { displayMode, toggleDisplayMode, setDisplayMode } = useRequestModeStore();
  const [sliderStyle, setSliderStyle] = useState({ width: 0, left: 0 });
  const itemsRef = useRef<{ [key: string]: HTMLButtonElement | null }>({});
  const containerRef = useRef<HTMLDivElement>(null);

  const updateSliderPosition = () => {
    const activeItem = itemsRef.current[displayMode];
    if (activeItem) {
      const { width, left } = activeItem.getBoundingClientRect();
      const parentLeft = activeItem.parentElement?.getBoundingClientRect().left || 0;
      setSliderStyle({
        width,
        left: left - parentLeft,
      });
    }
  };

  useEffect(() => {
    updateSliderPosition();

    const resizeObserver = new ResizeObserver(() => {
      updateSliderPosition();
    });

    if (containerRef.current) {
      resizeObserver.observe(containerRef.current);
    }

    return () => {
      resizeObserver.disconnect();
    };
  }, [displayMode]);

  return (
    <ToggleGroupRoot
      type="single"
      value={displayMode}
      className={cn("relative rounded-sm border border-[var(--moss-border-color)]", className)}
    >
      <div className="relative flex" ref={containerRef}>
        <div
          className="absolute h-[24px] rounded-sm bg-white transition-all duration-300 ease-in-out"
          style={{
            width: `${sliderStyle.width}px`,
            left: `${sliderStyle.left}px`,
          }}
        />
        <ToggleGroupItem
          value="REQUEST_FIRST"
          className="relative z-10 whitespace-nowrap transition-colors duration-300"
          compact={compact}
          ref={(el) => (itemsRef.current["REQUEST_FIRST"] = el)}
          onClick={() => setDisplayMode("REQUEST_FIRST")}
        >
          Request mode
        </ToggleGroupItem>
        <ToggleGroupItem
          value="DESIGN_FIRST"
          className="relative z-10 whitespace-nowrap transition-colors duration-300"
          compact={compact}
          ref={(el) => (itemsRef.current["DESIGN_FIRST"] = el)}
          onClick={() => setDisplayMode("DESIGN_FIRST")}
        >
          Design mode
        </ToggleGroupItem>
      </div>
    </ToggleGroupRoot>
  );
};

export default ModeToggle;
