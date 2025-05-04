import React from "react";
import type { SVGProps } from "react";
const SvgChevronDown: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M12.5 5.75L8 10.25L3.5 5.75"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgChevronDown;
