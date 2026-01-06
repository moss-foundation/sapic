import { useDeleteWorkspace, useListWorkspaces } from "@/adapters/tanstackQuery/workspace";
import { AppState } from "@/app/global/AppState";
import { useModal } from "@/hooks/useModal";
import { Icon, Menu } from "@/lib/ui";
import { useWelcomeOpenWorkspace } from "@/welcome/adapters/tanstackQuery/workspace";
import { ActionButton, ActionMenu, ConfirmationModal } from "@/workbench/ui/components";
import { NewWorkspaceModal } from "@/workbench/ui/components/Modals/Workspace/NewWorkspaceModal";
import { WorkspaceInfo } from "@repo/ipc";
import { Outlet, useLocation } from "@tanstack/react-router";

const WelcomeIndex = () => {
  const { data: workspaces } = useListWorkspaces();
  const location = useLocation();

  const {
    showModal: showNewWorkspaceModal,
    closeModal: closeNewWorkspaceModal,
    openModal: openNewWorkspaceModal,
  } = useModal();

  return (
    <AppState>
      <div className="flex h-screen w-full" data-tauri-drag-region>
        {/* Sidebar */}
        <div className="border-(--moss-border) flex w-64 flex-col border-r">
          <div className="flex flex-col gap-1 p-2">
            {/* Menu Items */}
            <div className="flex flex-col gap-0.5 pb-1">
              <a
                href="#/extensions"
                className={`py-1.25 text-(--moss-primary-foreground) hover:background-(--moss-toolbarItem-background-hover) flex items-center gap-2 rounded px-2 text-left ${
                  location.pathname === "/extensions" ? "background-(--moss-toolbarItem-background-hover)" : ""
                }`}
              >
                <Icon icon="Puzzle" className="size-4.5" />
                <span className="text-base">Extensions</span>
              </a>
              <a
                href="#/settings"
                className={`py-1.25 text-(--moss-primary-foreground) hover:background-(--moss-toolbarItem-background-hover) flex items-center gap-2 rounded px-2 text-left ${
                  location.pathname === "/settings" ? "background-(--moss-toolbarItem-background-hover)" : ""
                }`}
              >
                <Icon icon="Settings" className="size-4.5" />
                <span className="text-base">Settings</span>
              </a>
            </div>

            <Menu.Accordion defaultOpen>
              <div className="flex select-none items-center justify-between">
                <Menu.AccordionTrigger className="text-(--moss-secondary-foreground) flex-1 cursor-pointer px-0.5">
                  <span className="text-sm font-medium">Spaces</span>
                </Menu.AccordionTrigger>
                <ActionMenu.Root>
                  <ActionMenu.Trigger asChild>
                    <ActionButton icon="MoreHorizontal" hoverVariant="list" />
                  </ActionMenu.Trigger>
                  <ActionMenu.Portal>
                    <ActionMenu.Content>
                      <ActionMenu.Item onClick={openNewWorkspaceModal}>New workspace</ActionMenu.Item>
                    </ActionMenu.Content>
                  </ActionMenu.Portal>
                </ActionMenu.Root>
              </div>

              <Menu.AccordionContent>
                <div className="flex flex-col gap-1">
                  {workspaces && workspaces.length === 0 ? (
                    <div className="text-(--moss-secondary-foreground) py-1 text-sm">No workspaces</div>
                  ) : (
                    workspaces?.map((workspace) => <WorkspaceSidebarItem key={workspace.id} workspace={workspace} />)
                  )}
                </div>
              </Menu.AccordionContent>
            </Menu.Accordion>
          </div>
        </div>

        {/* Main Content */}
        <div className="flex flex-1 overflow-hidden">
          <Outlet />
        </div>
      </div>

      {showNewWorkspaceModal && (
        <NewWorkspaceModal
          closeModal={closeNewWorkspaceModal}
          showModal={showNewWorkspaceModal}
          forceNewWindow
          window="welcome"
        />
      )}
    </AppState>
  );
};

const WorkspaceSidebarItem = ({ workspace }: { workspace: WorkspaceInfo }) => {
  const { mutate: openWelcomeWorkspace } = useWelcomeOpenWorkspace();
  const { mutateAsync: deleteWorkspace } = useDeleteWorkspace();
  const {
    showModal: showDeleteWorkspaceModal,
    closeModal: closeDeleteWorkspaceModal,
    openModal: openDeleteWorkspaceModal,
  } = useModal();

  const firstLetter = workspace.name.charAt(0).toUpperCase();

  return (
    <>
      <div className="group/workspace flex items-center gap-2 rounded py-1 pl-1">
        <button
          onClick={() => openWelcomeWorkspace({ id: workspace.id })}
          className="size-4.5 flex shrink-0 cursor-pointer items-center justify-center rounded-sm bg-[#9333ea] text-xs font-medium text-white"
        >
          {firstLetter}
        </button>
        <button
          onClick={() => openWelcomeWorkspace({ id: workspace.id })}
          className="hover:text-(--moss-accent) flex-1 cursor-pointer text-left text-base font-medium transition duration-150 ease-in-out"
        >
          {workspace.name}
        </button>
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <button className="text-(--moss-toolbarItem-foreground) hover:background-(--moss-toolbarItem-background-hover) flex cursor-pointer items-center rounded p-1 opacity-0 transition-opacity group-hover/workspace:opacity-100">
              <Icon icon="MoreHorizontal" className="size-4" />
            </button>
          </ActionMenu.Trigger>
          <ActionMenu.Portal>
            <ActionMenu.Content>
              <ActionMenu.Item onClick={() => openWelcomeWorkspace({ id: workspace.id })}>Open</ActionMenu.Item>
              <ActionMenu.Item onClick={openDeleteWorkspaceModal}>Delete</ActionMenu.Item>
            </ActionMenu.Content>
          </ActionMenu.Portal>
        </ActionMenu.Root>
      </div>

      <ConfirmationModal
        title="Delete Workspace"
        message={`Are you sure you want to delete the workspace "${workspace.name}"?`}
        closeModal={closeDeleteWorkspaceModal}
        showModal={showDeleteWorkspaceModal}
        onConfirm={async () => await deleteWorkspace({ id: workspace.id })}
        onCancel={closeDeleteWorkspaceModal}
      />
    </>
  );
};

export default WelcomeIndex;
