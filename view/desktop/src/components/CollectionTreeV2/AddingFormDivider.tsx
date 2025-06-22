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
    }, 600);
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
      //prettier-ignore
      className={cn(`
          background-(--moss-primary) 
          absolute z-100 h-[2px] cursor-pointer 
          transition-opacity duration-100

          before:h-[5px] before:w-full before:content-[''] before:absolute before:left-0 before:-top-[5px]
          after:h-[5px] after:w-full after:content-[''] after:absolute after:left-0 after:-bottom-[5px]
         `,
        {
          "opacity-0": !visible,
          "-top-[1px] z-20": position === "top",
          "-bottom-[1px] z-30": position === "bottom",
        }
      )}
      style={{
        width: `calc(100% - ${paddingLeft}px - ${paddingRight}px )`,
        left: paddingLeft,
      }}
      onClick={visible ? handleClick : undefined}
    >
      <div className="relative h-full w-full">
        <div className="background-(--moss-drag-handle-bg) absolute -top-[8px] left-0 flex size-4 items-center justify-center rounded-sm p-px shadow">
          <DividerButtonIcon />
        </div>
      </div>
    </button>
  );
};

const DividerButtonIcon = () => {
  return (
    <svg width="8" height="8" viewBox="0 0 8 8" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M4.5 3.5V0H3.5V3.5H0V4.5H3.5V8H4.5V4.5H8V3.5H4.5Z" fill="#525252" />
    </svg>
  );
};
