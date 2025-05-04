import React from "react";
import type { SVGProps } from "react";
const SvgToolBarVariables: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M4 13H3C2.44772 13 2 12.5523 2 12V3C2 2.44771 2.44772 2 3 2H4"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-4)] dark:stroke-[var(--moss-gray-11)]"
    />
    <path
      d="M13 13H14C14.5523 13 15 12.5523 15 12V3C15 2.44771 14.5523 2 14 2H13"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-4)] dark:stroke-[var(--moss-gray-11)]"
    />
    <path
      d="M5 7.5H12"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-4)] dark:stroke-[var(--moss-gray-11)]"
    />
    <path
      d="M5 10H12"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-4)] dark:stroke-[var(--moss-gray-11)]"
    />
    <path d="M5 5H12" strokeLinecap="round" className="stroke-[var(--moss-gray-4)] dark:stroke-[var(--moss-gray-11)]" />
  </svg>
);
export default SvgToolBarVariables;
