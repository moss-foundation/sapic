import { FormEvent, useState } from "react";

import { Modal, PillTabs, Scrollbar } from "@/lib/ui";
import { invoke } from "@tauri-apps/api/core";
import { AddAccountParams, UpdateProfileInput } from "@repo/moss-app";
import { AccountKind } from "@repo/moss-user";

import { ModalWrapperProps } from "../../types";
import { ProviderIcon } from "./components";
import { FlowSection, FooterActions } from "./Sections";

interface NewAccountModalProps extends ModalWrapperProps {
  onAccountAdded?: () => void;
}

export const NewAccountModal = ({ showModal, closeModal, onAccountAdded }: NewAccountModalProps) => {
  const [provider, setProvider] = useState<AccountKind>("GITHUB");
  const [flow, setFlow] = useState<"OAUTH" | "PAT">("OAUTH");
  const [token, setToken] = useState("");
  const [useAsDefault, setUseAsDefault] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    try {
      setIsSubmitting(true);

      const accountParams: AddAccountParams = {
        host: provider === "GITHUB" ? "github.com" : "gitlab.com",
        label: "",
        kind: provider,
      };

      // If PAT flow is selected, we would add the token here
      if (flow === "PAT" && token) {
        console.log("PAT token:", token);
      }

      const input: UpdateProfileInput = {
        accountsToAdd: [accountParams],
        accountsToRemove: [],
      };

      await invoke("update_profile", { input });
      console.log("Account added successfully");

      onAccountAdded?.();
      handleClose();
      window.location.reload();
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
      setFlow("OAUTH");
      setToken("");
      setUseAsDefault(false);
    }, 200);
  };

  const isSubmitDisabled = isSubmitting || (flow === "PAT" && !token);

  return (
    <Modal onBackdropClick={handleClose} showModal={showModal} className="w-full max-w-[544px]">
      <form onSubmit={handleSubmit} className="flex flex-col overflow-hidden">
        <h2 className="flex items-center justify-center border-b border-(--moss-border-color) py-2 leading-4 font-medium">
          New Account
        </h2>

        <Scrollbar className="min-h-0 flex-1">
          <div className="flex flex-col px-6 pt-2 pb-5">
            <PillTabs.Root
              value={provider}
              onValueChange={(value) => setProvider(value as AccountKind)}
              className="flex flex-col gap-3.5"
            >
              <div className="flex items-center gap-3">
                <span>Provider:</span>
                <PillTabs.List>
                  <div className="flex gap-2">
                    <PillTabs.Trigger value="GITHUB" label="GitHub" leadingContent={<ProviderIcon icon="github" />} />
                    <PillTabs.Trigger value="GITLAB" label="GitLab" leadingContent={<ProviderIcon icon="gitlab" />} />
                  </div>
                </PillTabs.List>
              </div>

              <PillTabs.Content value="GITHUB">
                <FlowSection flow={flow} setFlow={setFlow} token={token} setToken={setToken} provider={provider} />
              </PillTabs.Content>
              <PillTabs.Content value="GITLAB">
                <FlowSection flow={flow} setFlow={setFlow} token={token} setToken={setToken} provider={provider} />
              </PillTabs.Content>
            </PillTabs.Root>
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
