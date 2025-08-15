import { FormEvent, useState } from "react";

import PaddedTabs from "@/components/PaddedTabs/PaddedTabs";
import { useCreateCollection } from "@/hooks/collection/useCreateCollection";
import { useStreamedCollections } from "@/hooks/collection/useStreamedCollections";
import { Modal } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { ModalWrapperProps } from "../../types";
import { CreateSection } from "./CreateSection/CreateSection";
import { Divider } from "./Divider";
import { FooterActions } from "./FooterActions";
import { Header } from "./Header";
import { ImportSection } from "./ImportSection/ImportSection";
import { ModeRadioGroup } from "./ModeRadioGroup";
import { Provider } from "./ProvidersRadioGroup/ProvidersRadioGroup";

interface NewCollectionModalProps extends ModalWrapperProps {
  initialTab?: "Create" | "Import";
}

export const NewCollectionModal = ({ closeModal, showModal, initialTab = "Create" }: NewCollectionModalProps) => {
  const { mutateAsync: createCollection } = useCreateCollection();
  const { data: collections } = useStreamedCollections();

  const { addOrFocusPanel } = useTabbedPaneStore();

  const [name, setName] = useState("New Collection");
  const [repository, setRepository] = useState("github.com/moss-foundation/sapic");
  const [mode, setMode] = useState<"Default" | "Custom">("Default");
  const [openAutomatically, setOpenAutomatically] = useState(true);

  const [tab, setTab] = useState<"Create" | "Import">(initialTab);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const result = await createCollection({
      name,
      repository,
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
  };

  const handleCancel = () => {
    closeModal();
  };

  const handleCreateSectionValuesUpdate = (values: { name: string; repository: string }) => {
    setName(values.name);
    setRepository(values.repository);
  };

  const handleImportSectionValuesUpdate = (values: {
    name: string;
    repository: string;
    branch: string;
    provider: Provider | null;
  }) => {
    setName(values.name);
    setRepository(values.repository);
  };

  const isSubmitDisabled = !name;

  return (
    <Modal onBackdropClick={handleCancel} showModal={showModal}>
      <Header />

      <Divider />

      <form onSubmit={handleSubmit} className="flex w-[544px] flex-col gap-1">
        <PaddedTabs.Root value={tab} onValueChange={(value) => setTab(value as "Create" | "Import")}>
          <PaddedTabs.List className="border-b border-(--moss-border-color) px-3">
            <PaddedTabs.Trigger value="Create">Create</PaddedTabs.Trigger>
            <PaddedTabs.Trigger value="Import">Import</PaddedTabs.Trigger>
          </PaddedTabs.List>

          <PaddedTabs.Content value="Create" className="px-6 pt-3">
            <CreateSection onValuesUpdate={handleCreateSectionValuesUpdate} />
          </PaddedTabs.Content>
          <PaddedTabs.Content value="Import" className="px-6 pt-3">
            <ImportSection onValuesUpdate={handleImportSectionValuesUpdate} />
          </PaddedTabs.Content>
        </PaddedTabs.Root>

        <div className="px-6 pb-3">
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
