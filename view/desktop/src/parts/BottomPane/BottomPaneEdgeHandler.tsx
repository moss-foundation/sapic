interface BottomPaneEdgeHandlerProps {
  onClick: () => void;
}

/**
 * A component that renders a handle at the bottom of the screen when the bottom pane is hidden.
 * Allows users to show the bottom pane by clicking on it.
 */
export const BottomPaneEdgeHandler = ({ onClick }: BottomPaneEdgeHandlerProps) => {
  return (
    <div
      className="group absolute right-0 bottom-0 left-0 z-10 h-3 cursor-pointer"
      onClick={onClick}
      title="Show bottom pane"
    >
      <div className="absolute inset-0 bg-[var(--moss-activityBar-indicator-color)] opacity-0 transition-opacity duration-200 group-hover:opacity-10" />
      <div className="absolute bottom-0.5 left-1/2 h-1 w-24 -translate-x-1/2 rounded-full bg-[var(--moss-activityBar-indicator-color)] opacity-30 transition-all duration-200 group-hover:h-1.5 group-hover:w-32 group-hover:opacity-70" />
      <div className="absolute bottom-1 left-1/2 -translate-x-1/2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
        <span className="bg-opacity-80 flex h-6 w-6 items-center justify-center rounded-full bg-[var(--moss-activityBar-indicator-color)] text-white">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
              d="M3 6L8 11L13 6"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </span>
      </div>
    </div>
  );
};

export default BottomPaneEdgeHandler;
