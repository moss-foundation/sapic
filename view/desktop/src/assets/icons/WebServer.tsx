import React from "react";
import type { SVGProps } from "react";
const SvgWebServer: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path d="M4 4H6V5H4V4Z" className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]" />
    <path
      d="M2 3V6C2 6.55228 2.44772 7 3 7H13C13.5523 7 14 6.55228 14 6V3C14 2.44772 13.5523 2 13 2H3C2.44772 2 2 2.44772 2 3ZM3 6V3H13L13 6H3Z"
      fillRule="evenodd"
      clipRule="evenodd"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
    <path d="M6 11H4V12H6V11Z" className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]" />
    <path
      d="M2 10V13C2 13.5523 2.44772 14 3 14H13C13.5523 14 14 13.5523 14 13V10C14 9.44772 13.5523 9 13 9H3C2.44772 9 2 9.44771 2 10ZM3 13V10H13L13 13H3Z"
      fillRule="evenodd"
      clipRule="evenodd"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgWebServer;
