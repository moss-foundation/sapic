import { FormEvent, useCallback, useState } from "react";

import { useCreateProject } from "@/hooks/project/useCreateProject";
import { useImportProject } from "@/hooks/project/useImportProject";
import { useStreamProjects } from "@/hooks/project/useStreamProjects";
import { Modal, Scrollbar } from "@/lib/ui";
import { UnderlinedTabs } from "@/lib/ui/Tabs/index";
import { useGitProviderStore } from "@/workbench/store/gitProvider";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { CreateProjectGitParams, ImportProjectSource } from "@repo/moss-workspace";

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
  const { data: projects } = useStreamProjects();
  const { mutateAsync: createProject } = useCreateProject();
  const { mutateAsync: importProject } = useImportProject();

  const { addOrFocusPanel } = useTabbedPaneStore();

  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState("New Project");
  const [mode, setMode] = useState<"Default" | "Custom">("Default");
  const [openAutomatically, setOpenAutomatically] = useState(true);
  const [createParams, setCreateParams] = useState<CreateProjectGitParams | undefined>(undefined);
  const [importParams, setImportParams] = useState<ImportProjectSource | undefined>(undefined);
  const [tab, setTab] = useState<typeof CREATE_TAB | typeof IMPORT_TAB>(initialTab);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (tab === CREATE_TAB) {
      const result = await createProject({
        name,
        gitParams: createParams,
        order: projects?.length ? projects.length + 1 : 1,
      });

      closeModal();

      if (openAutomatically) {
        addOrFocusPanel({
          id: result.id,
          title: result.name,
          component: "ProjectSettings",
          params: {
            projectId: result.id,
          },
        });
      }
    } else {
      if (!importParams) return;

      const result = await importProject({
        name,
        order: projects?.length ? projects.length + 1 : 1,
        source: importParams,
        iconPath: "",
      });

      closeModal();

      if (openAutomatically) {
        addOrFocusPanel({
          id: result.id,
          component: "ProjectSettings",
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

  const isSubmitDisabled = calculateIsSubmitDisabled({ name, tab, createParams, importParams, gitProvider });

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
