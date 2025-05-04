import React from "react";
import type { SVGProps } from "react";
const SvgChevronRight: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M6 11.5L9.5 8L6 4.5"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-7)] dark:stroke-[var(--moss-gray-10)]"
    />
  </svg>
);
export default SvgChevronRight;
