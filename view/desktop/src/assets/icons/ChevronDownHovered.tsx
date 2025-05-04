import React from "react";
import type { SVGProps } from "react";
const SvgChevronDownHovered: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <div {...props}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="block dark:hidden">
      <circle opacity="0.1" cx="8" cy="8" r="8" fill="#313547" />
      <path d="M11.5 6.25L8 9.75L4.5 6.25" stroke="#818594" strokeLinecap="round" />
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="hidden dark:block">
      <circle opacity="0.13" cx="8" cy="8" r="8" fill="#F0F1F2" />
      <path d="M11.5 6.25L8 9.75L4.5 6.25" stroke="#868A91" strokeLinecap="round" />
    </svg>
  </div>
);
export default SvgChevronDownHovered;
