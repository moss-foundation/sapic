import { PageContent } from "@/workbench/ui/components";

export const SettingsPage = () => {
  return (
    <PageContent className="flex flex-col gap-4">
      <div className="flex flex-col gap-2">
        <h1 className="text-2xl font-semibold">Settings</h1>
        <p className="text-(--moss-secondary-foreground)">Manage your settings</p>
      </div>
      {/* TODO: Add settings list and management UI */}
    </PageContent>
  );
};
