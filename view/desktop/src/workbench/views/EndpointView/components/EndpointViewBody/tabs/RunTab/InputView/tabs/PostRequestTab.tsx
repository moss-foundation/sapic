import { Icon } from "@/lib/ui";

export const PostRequestTabContent = () => {
  return (
    <div className="flex grow flex-col items-center justify-center gap-4 text-center opacity-60">
      <Icon icon="PostRequest" className="text-(--moss-gray-6) size-16" />

      <div className="flex flex-col items-center justify-center gap-2">
        <h3 className="text-(--moss-primary-foreground) text-lg font-medium">Post-Request Scripts</h3>
        <p className="text-(--moss-gray-6)">This section will contain JavaScript code executed after requests</p>
        <p className="text-(--moss-gray-6) text-sm">Coming soon...</p>
      </div>
    </div>
  );
};
