import { useRef, useState } from "react";

import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils/cn";

interface AddingFormDividerProps {
  paddingLeft: number;
  paddingRight: number;
  position: "top" | "bottom";
  onClick: () => void;
}

export const AddingFormDivider = ({ paddingLeft, paddingRight, position = "top", onClick }: AddingFormDividerProps) => {
  const [visible, setVisible] = useState(false);
  const timeoutIdRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const handleMouseEnter = () => {
    timeoutIdRef.current = setTimeout(() => {
      setVisible(true);
      timeoutIdRef.current = null;
    }, 300);
  };

  const handleMouseLeave = () => {
    if (timeoutIdRef.current) {
      clearTimeout(timeoutIdRef.current);
      timeoutIdRef.current = null;
    }
    setVisible(false);
  };

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.stopPropagation();
    onClick?.();
  };

  return (
    <button
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      className={cn(
        "background-(--moss-primary) absolute z-100 h-[2px] cursor-pointer transition-opacity duration-100",
        {
          "opacity-0": !visible,
          "top-0": position === "top",
          "bottom-0": position === "bottom",
        }
      )}
      style={{
        width: `calc(100% - ${paddingLeft}px - ${paddingRight}px )`,
        left: paddingLeft,
      }}
      onClick={visible ? handleClick : undefined}
    >
      <div className="relative h-full w-full">
        <div className="background-(--moss-primary) absolute -top-[8px] left-0 rounded-sm p-px">
          <Icon icon="Add" />
        </div>
      </div>
    </button>
  );
};
