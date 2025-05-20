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
      //prettier-ignore
      className={cn(`
          background-(--moss-primary) 
          absolute z-100 h-[2px] cursor-pointer 
          transition-opacity duration-100

          before:h-[3px] before:w-full before:content-[''] before:absolute before:left-0 before:-top-[3px]
          after:h-[3px] after:w-full after:content-[''] after:absolute after:left-0 after:-bottom-[3px]
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
        <div className="background-(--moss-primary) absolute -top-[8px] right-0 rounded-sm p-px">
          <DividerButtonIcon />
        </div>
      </div>
    </button>
  );
};

const DividerButtonIcon = () => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none" viewBox="0 0 24 24">
      <path fill="#fff" fill-opacity=".01" d="M0 0h24v24H0z" />
      <path
        fill="white"
        fill-rule="evenodd"
        d="M13 11V7.00195c0-.26521-.1054-.51957-.2929-.7071-.1876-.18754-.4419-.2929-.7071-.2929-.2653 0-.5196.10536-.7072.2929-.1875.18753-.2928.44189-.2928.7071V11H7.00195c-.26521 0-.51957.1053-.7071.2928-.18754.1876-.2929.4419-.2929.7072 0 .2652.10536.5195.2929.7071.18753.1875.44189.2929.7071.2929H11v3.998c0 .2652.1053.5195.2928.7071.1876.1875.4419.2929.7072.2929.2652 0 .5195-.1054.7071-.2929.1875-.1876.2929-.4419.2929-.7071V13h3.998c.2652 0 .5195-.1054.7071-.2929.1875-.1876.2929-.4419.2929-.7071 0-.2653-.1054-.5196-.2929-.7072-.1876-.1875-.4419-.2928-.7071-.2928H13Z"
        clip-rule="evenodd"
      />
    </svg>
  );
};
