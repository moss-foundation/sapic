import { Icon } from "@/lib/ui";

export const AuthTabContent = () => {
  return (
    <div className="flex grow flex-col items-center justify-center gap-4 text-center opacity-60">
      <Icon icon="Auth" className="text-gray-6 size-16" />

      <div className="flex flex-col items-center justify-center gap-2">
        <h3 className="text-gray-1 text-lg font-medium">Authentication Settings</h3>
        <p className="text-gray-6">This section will contain authentication configuration options</p>
        <p className="text-gray-6 text-sm">Coming soon...</p>
      </div>
    </div>
  );
};
