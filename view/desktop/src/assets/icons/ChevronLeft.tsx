import React from "react";
import type { SVGProps } from "react";
const SvgChevronLeft: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M9.5 11.5L6 8L9.5 4.5"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-7)] dark:stroke-[var(--moss-gray-10)]"
    />
  </svg>
);
export default SvgChevronLeft;
