import { useListWorkspaces, useOpenWorkspace } from "@/hooks";

export const WelcomePage = () => {
  const { data: workspaces } = useListWorkspaces();
  const { mutate: openWorkspace } = useOpenWorkspace();

  return (
    <div className="grid h-screen w-full place-items-center" data-tauri-drag-region>
      <div className="flex flex-col gap-6">
        <h1 className="text-3xl font-bold">Your workspaces</h1>
        <div className="flex flex-col gap-2">
          {workspaces && workspaces.length === 0 ? (
            <span className="text-(--moss-secondary-foreground)">No workspaces</span>
          ) : (
            <div className="flex flex-col items-start gap-1.5">
              {workspaces?.map((workspace) => (
                <button
                  key={workspace.id}
                  onClick={() => openWorkspace(workspace.id)}
                  className="cursor-pointer text-lg text-blue-500 hover:text-blue-600 hover:underline"
                >
                  {workspace.name} | id: {workspace.id}
                </button>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
