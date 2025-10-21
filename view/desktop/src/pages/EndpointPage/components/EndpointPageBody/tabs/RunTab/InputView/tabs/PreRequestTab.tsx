import { Icon } from "@/lib/ui";

export const PreRequestTabContent = () => {
  return (
    <div className="flex grow flex-col items-center justify-center gap-4 text-center opacity-60">
      <Icon icon="PreRequest" className="size-16 text-(--moss-gray-6)" />

      <div className="flex flex-col items-center justify-center gap-2">
        <h3 className="text-lg font-medium text-(--moss-gray-1)">Pre-Request Scripts</h3>
        <p className="text-(--moss-gray-6)">This section will contain JavaScript code executed before requests</p>
        <p className="text-sm text-(--moss-gray-6)">Coming soon...</p>
      </div>
    </div>
  );
};
