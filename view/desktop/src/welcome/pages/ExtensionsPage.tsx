import { PageContent } from "@/workbench/ui/components";

export const ExtensionsPage = () => {
  return (
    <PageContent className="flex flex-col gap-4">
      <div className="flex flex-col gap-2">
        <h1 className="text-2xl font-semibold">Extensions</h1>
        <p className="text-(--moss-secondary-foreground)">Manage your extensions</p>
      </div>
      {/* TODO: Add extensions list and management UI */}
    </PageContent>
  );
};
