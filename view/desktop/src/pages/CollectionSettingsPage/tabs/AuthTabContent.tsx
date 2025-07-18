import { IDockviewPanelProps } from "@/lib/moss-tabs/src";

export const AuthTabContent = ({}: IDockviewPanelProps<{ collectionId: string }>) => {
  return (
    <div className="flex h-full min-h-[400px] items-center justify-center">
      <div className="text-center opacity-60">
        <div className="mb-4">
          <svg
            className="mx-auto h-16 w-16 text-(--moss-secondary-text)"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={1.5}
              d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
            />
          </svg>
        </div>
        <h3 className="mb-2 text-lg font-medium text-(--moss-primary-text)">Authentication Settings</h3>
        <p className="text-(--moss-secondary-text)">This section will contain authentication configuration options</p>
        <p className="mt-1 text-sm text-(--moss-secondary-text)">Coming soon...</p>
      </div>
    </div>
  );
};
