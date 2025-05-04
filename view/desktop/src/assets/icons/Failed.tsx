import React from "react";
import type { SVGProps } from "react";
const SvgFailed: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <div {...props}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="block dark:hidden">
      <circle cx="8" cy="8" r="7" className="fill-[var(--moss-red-5)]" />
      <path d="M8 4.5L8 8" strokeWidth="1.8" strokeLinecap="round" className="stroke-[var(--moss-gray-14)]" />
      <circle cx="7.99844" cy="11.1" r="1.1" className="fill-[var(--moss-gray-14)]" />
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="hidden dark:block">
      <circle cx="8" cy="8" r="6.5" stroke="#DB5C5C" className="fill-[var(--moss-red-6)]" />
      <path d="M8 4.5L8 8" strokeWidth="1.8" strokeLinecap="round" className="stroke-[var(--moss-input-bg-outlined)]" />
      <circle cx="7.99844" cy="11.1" r="1.1" className="fill-[var(--moss-input-bg-outlined)]" />
    </svg>
  </div>
);
export default SvgFailed;
