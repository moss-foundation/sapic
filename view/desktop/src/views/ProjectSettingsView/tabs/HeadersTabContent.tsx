import { IDockviewPanelProps } from "moss-tabs";

export const HeadersTabContent = ({}: IDockviewPanelProps<{ projectId: string }>) => {
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
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
        </div>
        <h3 className="text-(--moss-primary-foreground) mb-2 text-lg font-medium">Headers Configuration</h3>
        <p className="text-(--moss-secondary-foreground)">
          This section will contain default headers for project requests
        </p>
        <p className="text-(--moss-secondary-foreground) mt-1 text-sm">Coming soon...</p>
      </div>
    </div>
  );
};
