import React, { useEffect, useRef, useState } from "react";

import { Item as ToggleGroupItem, Root as ToggleGroupRoot } from "@/components/ToggleGroup";
import { cn } from "@/utils";

type ToggleValue = "request" | "design";

interface ModeToggleProps {
  defaultValue?: ToggleValue;
  onValueChange?: (value: ToggleValue) => void;
  className?: string;
  compact?: boolean;
}

export const ModeToggle: React.FC<ModeToggleProps> = ({
  defaultValue = "request",
  onValueChange,
  className,
  compact = false,
}) => {
  const [value, setValue] = useState<ToggleValue>(defaultValue);
  const [sliderStyle, setSliderStyle] = useState({ width: 0, left: 0 });
  const itemsRef = useRef<{ [key: string]: HTMLButtonElement | null }>({});
  const containerRef = useRef<HTMLDivElement>(null);

  const updateSliderPosition = () => {
    const activeItem = itemsRef.current[value];
    if (activeItem) {
      const { width, left } = activeItem.getBoundingClientRect();
      const parentLeft = activeItem.parentElement?.getBoundingClientRect().left || 0;
      setSliderStyle({
        width,
        left: left - parentLeft,
      });
    }
  };

  const handleValueChange = (newValue: string) => {
    if (newValue === "request" || newValue === "design") {
      setValue(newValue as ToggleValue);
      onValueChange?.(newValue as ToggleValue);
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
  }, [value]);

  return (
    <ToggleGroupRoot
      type="single"
      value={value}
      onValueChange={handleValueChange}
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
          value="request"
          className="relative z-10 whitespace-nowrap transition-colors duration-300"
          compact={compact}
          ref={(el) => (itemsRef.current["request"] = el)}
        >
          Request-first mode
        </ToggleGroupItem>
        <ToggleGroupItem
          value="design"
          className="relative z-10 whitespace-nowrap transition-colors duration-300"
          compact={compact}
          ref={(el) => (itemsRef.current["design"] = el)}
        >
          Design-first mode
        </ToggleGroupItem>
      </div>
    </ToggleGroupRoot>
  );
};

export default ModeToggle;
