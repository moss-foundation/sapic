import React from "react";
import type { SVGProps } from "react";
const SvgConsole: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <div {...props}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="block dark:hidden">
      <rect x="1.5" y="2.5" width="13" height="11" rx="1.5" fill="#494B57" stroke="#494B57" />
      <path
        d="M7.98047 9.27441C8.13178 9.36198 8.15047 9.56438 8.03711 9.68066L7.98047 9.72461L4.83008 11.5439C4.65676 11.6439 4.44043 11.5185 4.44043 11.3184L4.44043 7.68164C4.44043 7.50645 4.60603 7.38822 4.76367 7.42871L4.83008 7.45605L7.98047 9.27441Z"
        stroke="#EBECF0"
        strokeWidth="0.88"
      />
      <rect x="1" y="5" width="14" height="1" fill="#EBECF0" />
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="hidden dark:block">
      <rect x="1.5" y="2.5" width="13" height="11" rx="1.5" fill="#CED0D6" stroke="#CED0D6" />
      <path
        d="M7.98047 9.27441C8.13178 9.36198 8.15047 9.56438 8.03711 9.68066L7.98047 9.72461L4.83008 11.5439C4.65676 11.6439 4.44043 11.5185 4.44043 11.3184L4.44043 7.68164C4.44043 7.50645 4.60603 7.38822 4.76367 7.42871L4.83008 7.45605L7.98047 9.27441Z"
        stroke="#43454A"
        strokeWidth="0.88"
      />
      <rect x="1.25" y="5.25" width="13.5" height="0.5" fill="#F7F8FA" stroke="#43454A" strokeWidth="0.5" />
    </svg>
  </div>
);
export default SvgConsole;
