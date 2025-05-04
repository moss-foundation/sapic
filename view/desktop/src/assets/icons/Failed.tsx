import React from "react";
import type { SVGProps } from "react";
const SvgFailed: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <div {...props}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="block dark:hidden">
      <circle cx="8" cy="8" r="7" fill="#E55765" />
      <path d="M8 4.5L8 8" stroke="white" strokeWidth="1.8" strokeLinecap="round" />
      <circle cx="7.99844" cy="11.1" r="1.1" fill="white" />
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="hidden dark:block">
      <circle cx="8" cy="8" r="6.5" fill="#DB5C5C" stroke="#DB5C5C" />
      <path d="M8 4.5L8 8" stroke="white" strokeWidth="1.8" strokeLinecap="round" />
      <circle cx="7.99844" cy="11.1" r="1.1" fill="white" />
    </svg>
  </div>
);
export default SvgFailed;
