import { IDockviewPanelProps } from "moss-tabs";

export const VariablesTabContent = ({}: IDockviewPanelProps<{ projectId: string }>) => {
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
              d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17v4a2 2 0 002 2h4M15 5l2 2m-2-2l2 2m-2-2L17 3"
            />
          </svg>
        </div>
        <h3 className="text-(--moss-primary-foreground) mb-2 text-lg font-medium">Variables Management</h3>
        <p className="text-(--moss-secondary-foreground)">
          This section will contain environment and project variables
        </p>
        <p className="text-(--moss-secondary-foreground) mt-1 text-sm">Coming soon...</p>
      </div>
    </div>
  );
};
