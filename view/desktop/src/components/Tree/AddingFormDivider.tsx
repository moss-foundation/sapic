import { useRef, useState } from "react";

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
          <DividerButtonIcon />
        </div>
      </div>
    </button>
  );
};

const DividerButtonIcon = () => {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        fill="white"
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M7.5 1C7.77614 1 8 1.22386 8 1.5V7H13.5C13.7761 7 14 7.22386 14 7.5C14 7.77614 13.7761 8 13.5 8H8V13.5C8 13.7761 7.77614 14 7.5 14C7.22386 14 7 13.7761 7 13.5V8H1.5C1.22386 8 1 7.77614 1 7.5C1 7.22386 1.22386 7 1.5 7H7V1.5C7 1.22386 7.22386 1 7.5 1Z"
      />
    </svg>
  );
};
