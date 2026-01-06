import { ProjectSettingsViewProps } from "../ProjectSettingsView";

export const PostRequestTabContent = ({}: ProjectSettingsViewProps) => {
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
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        </div>
        <h3 className="text-(--moss-primary-foreground) mb-2 text-lg font-medium">Post-Request Scripts</h3>
        <p className="text-(--moss-secondary-foreground)">
          This section will contain JavaScript code executed after receiving responses
        </p>
        <p className="text-(--moss-secondary-foreground) mt-1 text-sm">Coming soon...</p>
      </div>
    </div>
  );
};
