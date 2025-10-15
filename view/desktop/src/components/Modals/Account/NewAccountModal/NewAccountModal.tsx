import { FormEvent, useState } from "react";

import { VcsProviderSwitcher } from "@/components/VcsProviderSwitcher";
import { Modal, PillTabs, Scrollbar } from "@/lib/ui";
import { invoke } from "@tauri-apps/api/core";
import { AddAccountParams, UpdateProfileInput } from "@repo/moss-app";
import { AccountKind } from "@repo/moss-user";

import { ModalWrapperProps } from "../../types";
import { getProviderHost } from "../accountUtils";
import { MethodSection, FooterActions } from "./Sections";

interface NewAccountModalProps extends ModalWrapperProps {
  onAccountAdded?: () => void;
}

export const NewAccountModal = ({ showModal, closeModal, onAccountAdded }: NewAccountModalProps) => {
  const [provider, setProvider] = useState<AccountKind>("GITHUB");
  const [method, setMethod] = useState<"OAUTH" | "PAT">("OAUTH");
  const [token, setToken] = useState("");
  const [useAsDefault, setUseAsDefault] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    try {
      setIsSubmitting(true);

      const accountParams: AddAccountParams = {
        host: getProviderHost(provider),
        kind: provider,
        pat: method === "PAT" && token ? token : undefined,
      };

      const input: UpdateProfileInput = {
        accountsToAdd: [accountParams],
        accountsToRemove: [],
      };

      await invoke("update_profile", { input });
      console.log("Account added successfully");

      handleClose();
      onAccountAdded?.();
    } catch (error) {
      console.error("Error adding account:", error);
      alert(`Failed to add account: ${error}`);
    } finally {
      setIsSubmitting(false);
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

  const isSubmitDisabled = isSubmitting || (method === "PAT" && !token);

  return (
    <Modal onBackdropClick={handleClose} showModal={showModal} className="w-full max-w-136">
      <form onSubmit={handleSubmit} className="flex flex-col overflow-hidden">
        <h2 className="flex items-center justify-center border-b border-(--moss-border-color) py-2 leading-4 font-medium">
          New Account
        </h2>

        <Scrollbar className="min-h-0 flex-1">
          <div className="flex flex-col px-6 pt-2 pb-5">
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
          isSubmitting={isSubmitting}
        />
      </form>
    </Modal>
  );
};
