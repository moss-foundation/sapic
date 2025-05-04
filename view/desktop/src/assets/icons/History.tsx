import React from "react";
import type { SVGProps } from "react";
const SvgHistory: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <circle cx="8" cy="8" r="6.5" className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]" />
    <path
      d="M8 5V8L10.5 9.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgHistory;
