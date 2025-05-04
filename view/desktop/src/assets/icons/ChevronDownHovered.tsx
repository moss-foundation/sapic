import React from "react";
import type { SVGProps } from "react";
const SvgChevronDownHovered: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <div {...props}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="block dark:hidden">
      <circle opacity="0.1" cx="8" cy="8" r="8" className="fill-[var(--moss-gray-4)]" />
      <path d="M11.5 6.25L8 9.75L4.5 6.25" strokeLinecap="round" className="stroke-[var(--moss-gray-7)]" />
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="hidden dark:block">
      <circle opacity="0.13" cx="8" cy="8" r="8" className="fill-[var(--moss-gray-13)]" />
      <path d="M11.5 6.25L8 9.75L4.5 6.25" strokeLinecap="round" className="stroke-[var(--moss-gray-8)]" />
    </svg>
  </div>
);
export default SvgChevronDownHovered;
