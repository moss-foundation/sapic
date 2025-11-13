import { Icon } from "@/lib/ui";

export const BodyTabContent = () => {
  return (
    <div className="flex grow flex-col items-center justify-center gap-4 text-center opacity-60">
      <Icon icon="Braces" className="text-gray-6 size-16" />

      <div className="flex flex-col items-center justify-center gap-2">
        <h3 className="text-gray-1 text-lg font-medium">Body</h3>
        <p className="text-gray-6">This section will contain endpoint body configuration</p>
        <p className="text-gray-6 text-sm">Coming soon...</p>
      </div>
    </div>
  );
};
