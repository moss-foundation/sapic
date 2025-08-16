import { FormEvent, useState } from "react";

import PaddedTabs from "@/components/PaddedTabs/PaddedTabs";
import { useCreateCollection } from "@/hooks/collection/useCreateCollection";
import { useImportCollection } from "@/hooks/collection/useImportCollection";
import { useStreamedCollections } from "@/hooks/collection/useStreamedCollections";
import { Modal } from "@/lib/ui";
import { useGitProviderStore } from "@/store/gitProvider";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { CreateCollectionGitParams, ImportCollectionSource } from "@repo/moss-workspace";

import { ModalWrapperProps } from "../../types";
import { CreateSection } from "./CreateSection/CreateSection";
import { Divider } from "./Divider";
import { FooterActions } from "./FooterActions";
import { Header } from "./Header";
import { ImportSection } from "./ImportSection/ImportSection";
import { ModeRadioGroup } from "./ModeRadioGroup";
import { calculateIsSubmitDisabled } from "./utils";

interface NewCollectionModalProps extends ModalWrapperProps {
  initialTab?: "Create" | "Import";
}

export const NewCollectionModal = ({ closeModal, showModal, initialTab = "Create" }: NewCollectionModalProps) => {
  const { mutateAsync: createCollection } = useCreateCollection();
  const { data: collections } = useStreamedCollections();
  const { mutateAsync: importCollection } = useImportCollection();

  const { addOrFocusPanel } = useTabbedPaneStore();

  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState("New Collection");
  const [mode, setMode] = useState<"Default" | "Custom">("Default");
  const [openAutomatically, setOpenAutomatically] = useState(true);
  const [createParams, setCreateParams] = useState<CreateCollectionGitParams | undefined>(undefined);
  const [importParams, setImportParams] = useState<ImportCollectionSource | undefined>(undefined);
  const [tab, setTab] = useState<"Create" | "Import">(initialTab);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (tab === "Create") {
      const result = await createCollection({
        name,
        gitParams: createParams,
        order: collections?.length ? collections.length + 1 : 1,
      });

      closeModal();

      if (openAutomatically) {
        addOrFocusPanel({
          id: result.id,
          component: "CollectionSettings",
          params: {
            collectionId: result.id,
          },
        });
      }
    } else {
      if (!importParams) {
        // throw new Error("Import params are required");
        return;
      }

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

  const handleCreateSectionValuesUpdate = (values: {
    name: string;
    gitParams: CreateCollectionGitParams | undefined;
  }) => {
    setName(values.name);
    setCreateParams(values.gitParams);
  };

  const handleImportSectionValuesUpdate = (values: {
    name: string;
    importParams: ImportCollectionSource | undefined;
  }) => {
    setName(values.name);
    setImportParams(values.importParams);
  };

  const isSubmitDisabled = calculateIsSubmitDisabled({ name, tab, createParams, importParams, gitProvider });

  return (
    <Modal onBackdropClick={handleCancel} showModal={showModal}>
      <Header />

      <Divider />

      <form onSubmit={handleSubmit} className="flex w-[544px] flex-col">
        <PaddedTabs.Root value={tab} onValueChange={(value) => setTab(value as "Create" | "Import")}>
          <PaddedTabs.List className="border-b border-(--moss-border-color) px-3">
            <PaddedTabs.Trigger value="Create">Create</PaddedTabs.Trigger>
            <PaddedTabs.Trigger value="Import">Import</PaddedTabs.Trigger>
          </PaddedTabs.List>

          <PaddedTabs.Content value="Create" className="px-6 py-3">
            <CreateSection onValuesUpdate={handleCreateSectionValuesUpdate} />
          </PaddedTabs.Content>
          <PaddedTabs.Content value="Import" className="px-6 py-3">
            <ImportSection onValuesUpdate={handleImportSectionValuesUpdate} />
          </PaddedTabs.Content>
        </PaddedTabs.Root>

        <div className="px-6 pb-6">
          <ModeRadioGroup mode={mode} setMode={setMode} />
        </div>

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
