import { IDockviewPanelProps } from "moss-tabs";

export const PreRequestTabContent = ({}: IDockviewPanelProps<{ projectId: string }>) => {
  return (
    <div className="flex h-full min-h-[400px] items-center justify-center">
      <div className="text-center opacity-60">
        <div className="mb-4">
          <svg
            className="text-(--moss-secondary-foreground) mx-auto h-16 w-16"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={1.5}
              d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
            />
          </svg>
        </div>
        <h3 className="text-(--moss-primary-foreground) mb-2 text-lg font-medium">Pre-Request Scripts</h3>
        <p className="text-(--moss-secondary-foreground)">
          This section will contain JavaScript code executed before requests
        </p>
        <p className="text-(--moss-secondary-foreground) mt-1 text-sm">Coming soon...</p>
      </div>
    </div>
  );
};
