import React from "react";
import type { SVGProps } from "react";
const SvgSend: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <div {...props}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 17 16" fill="none" className="block dark:hidden">
      <path d="M10.5 8H4.5L3.5 14.5L15.5 8L3.5 1.5L4.5 8" fill="#EDF3FF" />
      <path
        d="M10.5 8H4.5M4.5 8L3.5 14.5L15.5 8L3.5 1.5L4.5 8Z"
        stroke="#3574F0"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="hidden dark:block">
      <path d="M10.5 8H4.5L3.5 14.5L15.5 8L3.5 1.5L4.5 8" fill="#25324D" />
      <path
        d="M10.5 8H4.5M4.5 8L3.5 14.5L15.5 8L3.5 1.5L4.5 8Z"
        stroke="#548AF7"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  </div>
);
export default SvgSend;
