import React from "react";
import type { SVGProps } from "react";
const SvgPersistenceRelationship: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <circle cx="8" cy="8" r="4.5" className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]" />
    <circle
      cx="11.5"
      cy="4.5"
      r="3"
      className="fill-[var(--moss-orange-9)] stroke-[var(--moss-orange-4)] dark:fill-[var(--moss-orange-1)] dark:stroke-[var(--moss-orange-5)]"
    />
    <circle
      cx="4.5"
      cy="11.5"
      r="3"
      className="fill-[var(--moss-orange-9)] stroke-[var(--moss-orange-4)] dark:fill-[var(--moss-orange-1)] dark:stroke-[var(--moss-orange-5)]"
    />
  </svg>
);
export default SvgPersistenceRelationship;
