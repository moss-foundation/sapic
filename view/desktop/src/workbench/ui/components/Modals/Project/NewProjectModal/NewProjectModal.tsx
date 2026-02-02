import { FormEvent, useCallback, useState } from "react";

import { useCreateProject } from "@/adapters/tanstackQuery/project/useCreateProject";
import { useImportProject } from "@/adapters/tanstackQuery/project/useImportProject";
import { useStreamProjects } from "@/adapters/tanstackQuery/project/useStreamProjects";
import { useCurrentWorkspace } from "@/hooks";
import { Modal, Scrollbar } from "@/lib/ui";
import { UnderlinedTabs } from "@/lib/ui/Tabs/index";
import { usePutTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useUpdateTreeItemState";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { CreateProjectGitParams, ImportProjectSource } from "@repo/ipc";

import { ModalWrapperProps } from "../../types";
import { Divider } from "./components/Divider";
import { ModeRadioGroup } from "./components/ModeRadioGroup";
import { CREATE_TAB, IMPORT_TAB } from "./constansts";
import { CreateSection, FooterActions, ImportSection } from "./Sections";
import { calculateIsSubmitDisabled } from "./utils";

interface NewProjectModalProps extends ModalWrapperProps {
  initialTab?: typeof CREATE_TAB | typeof IMPORT_TAB;
}

export const NewProjectModal = ({ closeModal, showModal, initialTab = CREATE_TAB }: NewProjectModalProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: projects } = useStreamProjects();
  const { mutateAsync: createProject } = useCreateProject();
  const { mutateAsync: importProject } = useImportProject();

  const { mutateAsync: updateTreeItemState } = usePutTreeItemState();

  const { addOrFocusPanel } = useTabbedPaneStore();

  const [name, setName] = useState("New Project");
  const [mode, setMode] = useState<"Default" | "Custom">("Default");
  const [openAutomatically, setOpenAutomatically] = useState(true);
  const [createParams, setCreateParams] = useState<CreateProjectGitParams | undefined>(undefined);
  const [importParams, setImportParams] = useState<ImportProjectSource | undefined>(undefined);
  const [tab, setTab] = useState<typeof CREATE_TAB | typeof IMPORT_TAB>(initialTab);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const newOrder = projects?.length ? projects.length + 1 : 1;

    if (tab === CREATE_TAB) {
      const result = await createProject({
        name,
        gitParams: createParams,
        order: newOrder,
      });

      await updateTreeItemState({
        treeItemState: {
          id: result.id,
          order: newOrder,
          expanded: true,
        },
        workspaceId: currentWorkspaceId,
      });

      closeModal();

      if (openAutomatically) {
        addOrFocusPanel({
          id: result.id,
          title: result.name,
          component: "ProjectSettingsView",
          params: {
            projectId: result.id,
          },
        });
      }
    } else {
      if (!importParams) return;

      const result = await importProject({
        name,
        order: newOrder,
        source: importParams,
        iconPath: "",
      });

      await updateTreeItemState({
        treeItemState: {
          id: result.id,
          order: newOrder,
          expanded: true,
        },
        workspaceId: currentWorkspaceId,
      });

      closeModal();

      if (openAutomatically) {
        addOrFocusPanel({
          id: result.id,
          component: "ProjectSettingsView",
          title: result.name,
          params: {
            projectId: result.id,
          },
        });
      }
    }
  };

  const handleCancel = () => {
    closeModal();
  };

  const handleCreateSectionValuesUpdate = useCallback(
    (values: { name: string; gitParams: CreateProjectGitParams | undefined }) => {
      setName(values.name);
      setCreateParams(values.gitParams);
    },
    []
  );

  const handleImportSectionValuesUpdate = useCallback(
    (values: { name: string; importParams: ImportProjectSource | undefined }) => {
      setName(values.name);
      setImportParams(values.importParams);
    },
    []
  );

  const isSubmitDisabled = calculateIsSubmitDisabled({ name, tab, createParams, importParams });

  return (
    <Modal onBackdropClick={handleCancel} showModal={showModal} className="max-w-136 w-full">
      <h2 className="flex items-center justify-center py-2 font-medium leading-4">New Project</h2>

      <Divider />

      <form onSubmit={handleSubmit} className="flex flex-col overflow-hidden">
        <Scrollbar className="min-h-0 flex-1">
          <div className="flex flex-col">
            <UnderlinedTabs.Root
              value={tab}
              onValueChange={(value) => setTab(value as typeof CREATE_TAB | typeof IMPORT_TAB)}
            >
              <UnderlinedTabs.List className="border-(--moss-border) border-b px-3">
                <UnderlinedTabs.Trigger value={CREATE_TAB}>Create</UnderlinedTabs.Trigger>
                <UnderlinedTabs.Trigger value={IMPORT_TAB}>Import</UnderlinedTabs.Trigger>
              </UnderlinedTabs.List>

              <UnderlinedTabs.Content value={CREATE_TAB} className="px-6 py-3">
                <CreateSection onValuesUpdate={handleCreateSectionValuesUpdate} />
              </UnderlinedTabs.Content>
              <UnderlinedTabs.Content value={IMPORT_TAB} className="px-6 py-3">
                <ImportSection onValuesUpdate={handleImportSectionValuesUpdate} />
              </UnderlinedTabs.Content>
            </UnderlinedTabs.Root>

            <div className="px-6 pb-6">
              <ModeRadioGroup mode={mode} setMode={setMode} />
            </div>
          </div>
        </Scrollbar>

        <FooterActions
          openAutomatically={openAutomatically}
          setOpenAutomatically={setOpenAutomatically}
          handleCancel={handleCancel}
          isSubmitDisabled={isSubmitDisabled}
          tab={tab}
        />
      </form>
    </Modal>
  );
};
