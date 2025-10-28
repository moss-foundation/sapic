import { Icon } from "@/lib/ui";

export const HeadersTabContent = () => {
  return (
    <div className="flex grow flex-col items-center justify-center gap-4 text-center opacity-60">
      <Icon icon="Headers" className="text-(--moss-gray-6) size-16" />

      <div className="flex flex-col items-center justify-center gap-2">
        <h3 className="text-(--moss-gray-1) text-lg font-medium">Headers</h3>
        <p className="text-(--moss-gray-6)">This section will contain endpoint headers configuration</p>
        <p className="text-(--moss-gray-6) text-sm">Coming soon...</p>
      </div>
    </div>
  );
};
