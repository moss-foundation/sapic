import { FormEvent, useCallback, useState } from "react";

import PaddedTabs from "@/components/PaddedTabs/PaddedTabs";
import { useCreateCollection } from "@/hooks/collection/useCreateCollection";
import { useImportCollection } from "@/hooks/collection/useImportCollection";
import { useStreamCollections } from "@/hooks/collection/useStreamCollections";
import { Modal, Scrollbar } from "@/lib/ui";
import { useGitProviderStore } from "@/store/gitProvider";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { CreateCollectionGitParams, ImportCollectionSource } from "@repo/moss-workspace";

import { ModalWrapperProps } from "../../types";
import { Divider } from "./components/Divider";
import { ModeRadioGroup } from "./components/ModeRadioGroup";
import { CREATE_TAB, IMPORT_TAB } from "./constansts";
import { CreateSection, FooterActions, Header, ImportSection } from "./Sections";
import { calculateIsSubmitDisabled } from "./utils";

interface NewCollectionModalProps extends ModalWrapperProps {
  initialTab?: typeof CREATE_TAB | typeof IMPORT_TAB;
}

export const NewCollectionModal = ({ closeModal, showModal, initialTab = CREATE_TAB }: NewCollectionModalProps) => {
  const { data: collections } = useStreamCollections();
  const { mutateAsync: createCollection } = useCreateCollection();
  const { mutateAsync: importCollection } = useImportCollection();

  const { addOrFocusPanel } = useTabbedPaneStore();

  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState("New Collection");
  const [mode, setMode] = useState<"Default" | "Custom">("Default");
  const [openAutomatically, setOpenAutomatically] = useState(true);
  const [createParams, setCreateParams] = useState<CreateCollectionGitParams | undefined>(undefined);
  const [importParams, setImportParams] = useState<ImportCollectionSource | undefined>(undefined);
  const [tab, setTab] = useState<typeof CREATE_TAB | typeof IMPORT_TAB>(initialTab);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (tab === CREATE_TAB) {
      const result = await createCollection({
        name,
        gitParams: createParams,
        order: collections?.length ? collections.length + 1 : 1,
      });

      closeModal();

      if (openAutomatically) {
        addOrFocusPanel({
          id: result.id,
          title: result.name,
          component: "CollectionSettings",
          params: {
            collectionId: result.id,
          },
        });
      }
    } else {
      if (!importParams) return;

      const result = await importCollection({
        name,
        order: collections?.length ? collections.length + 1 : 1,
        source: importParams,
        //TODO this is hardcoded, but we don't have and interface for this yet
        externalPath: "",
        iconPath: "",
      });

      closeModal();

      if (openAutomatically) {
        addOrFocusPanel({
          id: result.id,
          component: "CollectionSettings",
          title: result.name,
          params: {
            collectionId: result.id,
          },
        });
      }
    }
  };

  const handleCancel = () => {
    closeModal();
  };

  const handleCreateSectionValuesUpdate = useCallback(
    (values: { name: string; gitParams: CreateCollectionGitParams | undefined }) => {
      setName(values.name);
      setCreateParams(values.gitParams);
    },
    []
  );

  const handleImportSectionValuesUpdate = useCallback(
    (values: { name: string; importParams: ImportCollectionSource | undefined }) => {
      setName(values.name);
      setImportParams(values.importParams);
    },
    []
  );

  const isSubmitDisabled = calculateIsSubmitDisabled({ name, tab, createParams, importParams, gitProvider });

  return (
    <Modal onBackdropClick={handleCancel} showModal={showModal} className="w-full max-w-[544px]">
      <Header />

      <Divider />

      <form onSubmit={handleSubmit} className="flex flex-col overflow-hidden">
        <Scrollbar className="min-h-0 flex-1">
          <div className="flex flex-col">
            <PaddedTabs.Root
              value={tab}
              onValueChange={(value) => setTab(value as typeof CREATE_TAB | typeof IMPORT_TAB)}
            >
              <PaddedTabs.List className="border-b border-(--moss-border-color) px-3">
                <PaddedTabs.Trigger value={CREATE_TAB}>Create</PaddedTabs.Trigger>
                <PaddedTabs.Trigger value={IMPORT_TAB}>Import</PaddedTabs.Trigger>
              </PaddedTabs.List>

              <PaddedTabs.Content value={CREATE_TAB} className="px-6 py-3">
                <CreateSection onValuesUpdate={handleCreateSectionValuesUpdate} />
              </PaddedTabs.Content>
              <PaddedTabs.Content value={IMPORT_TAB} className="px-6 py-3">
                <ImportSection onValuesUpdate={handleImportSectionValuesUpdate} />
              </PaddedTabs.Content>
            </PaddedTabs.Root>

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
