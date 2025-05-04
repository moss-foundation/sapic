import React from "react";
import type { SVGProps } from "react";
const SvgFind: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <circle cx="7" cy="7" r="4.5" className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]" />
    <path
      d="M10.1992 10.1992L13.4992 13.4952"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgFind;
