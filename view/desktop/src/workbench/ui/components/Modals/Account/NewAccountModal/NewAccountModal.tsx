import { FormEvent, useState } from "react";

import { useUpdateProfile } from "@/adapters";
import { Modal, PillTabs, Scrollbar } from "@/lib/ui";
import { VcsProviderSwitcher } from "@/workbench/ui/components/VcsProviderSwitcher";
import { AccountKind } from "@repo/moss-user";
import { AddAccountParams, UpdateProfileInput } from "@repo/window";

import { ModalWrapperProps } from "../../types";
import { getProviderHost } from "../accountUtils";
import { FooterActions, MethodSection } from "./Sections";

interface NewAccountModalProps extends ModalWrapperProps {
  onAccountAdded?: () => void;
}

export const NewAccountModal = ({ showModal, closeModal, onAccountAdded }: NewAccountModalProps) => {
  const { mutateAsync: updateProfile, isPending: isUpdatingProfile } = useUpdateProfile();

  const [provider, setProvider] = useState<AccountKind>("GITHUB");
  const [method, setMethod] = useState<"OAUTH" | "PAT">("OAUTH");
  const [token, setToken] = useState("");
  const [useAsDefault, setUseAsDefault] = useState(false);

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    try {
      const accountParams: AddAccountParams = {
        host: getProviderHost(provider),
        kind: provider,
        pat: method === "PAT" && token ? token : undefined,
      };

      const input: UpdateProfileInput = {
        accountsToAdd: [accountParams],
        accountsToRemove: [],
        accountsToUpdate: [],
      };

      await updateProfile(input);

      handleClose();
      onAccountAdded?.();
    } catch (error) {
      console.error("Error adding account:", error);
      alert(`Failed to add account: ${error}`);
    }
  };

  const handleClose = () => {
    closeModal();
    reset();
  };

  const reset = () => {
    setTimeout(() => {
      setProvider("GITHUB");
      setMethod("OAUTH");
      setToken("");
      setUseAsDefault(false);
    }, 200);
  };

  const isSubmitDisabled = isUpdatingProfile || (method === "PAT" && !token);

  return (
    <Modal onBackdropClick={handleClose} showModal={showModal} className="max-w-136 w-full">
      <form onSubmit={handleSubmit} className="flex flex-col overflow-hidden">
        <h2 className="border-(--moss-border) flex items-center justify-center border-b py-2 font-medium leading-4">
          New Account
        </h2>

        <Scrollbar className="min-h-0 flex-1">
          <div className="flex flex-col px-6 pb-5 pt-2">
            <VcsProviderSwitcher
              value={provider}
              onValueChange={(value) => setProvider(value.toUpperCase() as AccountKind)}
              label="Provider:"
              layout="vertical"
            >
              <PillTabs.Content value="github">
                <MethodSection
                  method={method}
                  setMethod={setMethod}
                  token={token}
                  setToken={setToken}
                  provider={provider}
                />
              </PillTabs.Content>
              <PillTabs.Content value="gitlab">
                <MethodSection
                  method={method}
                  setMethod={setMethod}
                  token={token}
                  setToken={setToken}
                  provider={provider}
                />
              </PillTabs.Content>
            </VcsProviderSwitcher>
          </div>
        </Scrollbar>

        <FooterActions
          useAsDefault={useAsDefault}
          setUseAsDefault={setUseAsDefault}
          handleCancel={handleClose}
          isSubmitDisabled={isSubmitDisabled}
          isSubmitting={isUpdatingProfile}
        />
      </form>
    </Modal>
  );
};
