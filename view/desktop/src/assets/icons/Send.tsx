import React from "react";
import type { SVGProps } from "react";
const SvgSend: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <div {...props}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 17 16" fill="none" className="block dark:hidden">
      <path d="M10.5 8H4.5L3.5 14.5L15.5 8L3.5 1.5L4.5 8" className="fill-[var(--moss-blue-12)]" />
      <path
        d="M10.5 8H4.5M4.5 8L3.5 14.5L15.5 8L3.5 1.5L4.5 8Z"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="stroke-[var(--moss-blue-4)]"
      />
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="hidden dark:block">
      <path d="M10.5 8H4.5L3.5 14.5L15.5 8L3.5 1.5L4.5 8" className="fill-[var(--moss-blue-1)]" />
      <path
        d="M10.5 8H4.5M4.5 8L3.5 14.5L15.5 8L3.5 1.5L4.5 8Z"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="stroke-[var(--moss-blue-8)]"
      />
    </svg>
  </div>
);
export default SvgSend;
