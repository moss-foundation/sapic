interface SidebarEdgeHandlerProps {
  position: string;
  onClick: () => void;
  activityBarOffset?: number;
}

/**
 * A component that renders a handle at the edge of the screen when the sidebar is hidden.
 * Allows users to show the sidebar by clicking on it.
 */
export const SidebarEdgeHandler = ({ position, onClick, activityBarOffset = 0 }: SidebarEdgeHandlerProps) => {
  const isLeft = position === "left";

  return (
    <div
      className={`group absolute top-0 bottom-0 z-10 w-3 cursor-pointer ${isLeft ? "left-0" : "right-0"}`}
      onClick={onClick}
      style={{
        [isLeft ? "marginLeft" : "marginRight"]: `${activityBarOffset}px`,
      }}
      title={`Show ${position} sidebar`}
    >
      <div
        className={`absolute inset-0 bg-[var(--moss-activityBar-indicator-color)] opacity-0 transition-opacity duration-200 group-hover:opacity-10`}
      />

      <div
        className={`absolute ${isLeft ? "left-0.5" : "right-0.5"} top-1/2 h-24 w-1 -translate-y-1/2 rounded-full bg-[var(--moss-activityBar-indicator-color)] opacity-30 transition-all duration-200 group-hover:h-32 group-hover:w-1.5 group-hover:opacity-70`}
      />

      <div
        className={`absolute ${isLeft ? "left-1" : "right-1"} top-1/2 -translate-y-1/2 opacity-0 transition-opacity duration-300 group-hover:opacity-100`}
      >
        <span
          className={`bg-opacity-80 flex h-6 w-6 items-center justify-center rounded-full bg-[var(--moss-activityBar-indicator-color)] text-white`}
        >
          {isLeft ? (
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path
                d="M10 3L5 8L10 13"
                stroke="currentColor"
                strokeWidth="1.5"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          ) : (
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path
                d="M6 3L11 8L6 13"
                stroke="currentColor"
                strokeWidth="1.5"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          )}
        </span>
      </div>
    </div>
  );
};

export default SidebarEdgeHandler;
