import { IDockviewPanelProps } from "@repo/moss-tabs";

export const PostRequestTabContent = ({}: IDockviewPanelProps<{
  node?: any;
  treeId: string;
  iconType: any;
  someRandomString: string;
}>) => {
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
              d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
            />
          </svg>
        </div>
        <h3 className="mb-2 text-lg font-medium text-(--moss-primary-text)">Post-Request Scripts</h3>
        <p className="text-(--moss-secondary-text)">
          This section will contain JavaScript code executed after requests
        </p>
        <p className="mt-1 text-sm text-(--moss-secondary-text)">Coming soon...</p>
      </div>
    </div>
  );
};
