import React from "react";
import type { SVGProps } from "react";
const SvgMoreHorizontal: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <circle
      cx="3"
      cy="8"
      r="1"
      transform="rotate(-90 3 8)"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
    <circle
      cx="8"
      cy="8"
      r="1"
      transform="rotate(-90 8 8)"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
    <circle
      cx="13"
      cy="8"
      r="1"
      transform="rotate(-90 13 8)"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgMoreHorizontal;
