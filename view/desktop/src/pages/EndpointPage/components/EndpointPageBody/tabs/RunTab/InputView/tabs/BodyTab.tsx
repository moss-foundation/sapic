import { Icon } from "@/lib/ui";

export const BodyTabContent = () => {
  return (
    <div className="flex grow flex-col items-center justify-center gap-4 text-center opacity-60">
      <Icon icon="Braces" className="size-16 text-(--moss-secondary-text)" />

      <div className="flex flex-col items-center justify-center gap-2">
        <h3 className="text-lg font-medium text-(--moss-primary-text)">Body</h3>
        <p className="text-(--moss-secondary-text)">This section will contain endpoint body configuration</p>
        <p className="text-sm text-(--moss-secondary-text)">Coming soon...</p>
      </div>
    </div>
  );
};
