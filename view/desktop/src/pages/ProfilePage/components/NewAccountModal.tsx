import { useRef, useState } from "react";

import { InputOutlined, RadioGroup } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { ModalForm } from "@/components/ModalForm";
import { invoke } from "@tauri-apps/api/core";
import { AddAccountParams, UpdateProfileInput } from "@repo/moss-app";
import { AccountKind } from "@repo/moss-user";

interface NewAccountModalProps {
  showModal: boolean;
  closeModal: () => void;
  onAccountAdded?: () => void;
}

export const NewAccountModal = ({ showModal, closeModal, onAccountAdded }: NewAccountModalProps) => {
  const tokenInputRef = useRef<HTMLInputElement>(null);

  const [provider, setProvider] = useState<AccountKind>("GITHUB");
  const [flow, setFlow] = useState<"OAUTH" | "PAT">("OAUTH");
  const [token, setToken] = useState("");
  const [useAsDefault, setUseAsDefault] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async () => {
    try {
      setIsSubmitting(true);

      const accountParams: AddAccountParams = {
        host: provider === "GITHUB" ? "github.com" : "gitlab.com",
        label: "",
        kind: provider,
      };

      // If PAT flow is selected, we would add the token here
      // For now, we're focusing on OAuth flow
      if (flow === "PAT" && token) {
        // In a real implementation, this would include the PAT
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

  return (
    <ModalForm
      title="New Account"
      onBackdropClick={handleClose}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background) max-w-[544px]"
      titleClassName="border-b border-(--moss-border-color)"
      footerClassName="border-t border-(--moss-border-color)"
      content={
        <div className="flex flex-col gap-2">
          {/* Provider Selection */}
          <div className="grid grid-cols-[min-content_1fr] items-start gap-x-3.75 py-4">
            <div className="pt-2">Provider:</div>
            <div className="flex gap-2">
              <button
                type="button"
                onClick={() => setProvider("GITHUB")}
                className={`flex items-center gap-2 rounded-full border px-4 py-1 transition-colors ${
                  provider === "GITHUB"
                    ? "background-(--moss-primary) border-(--moss-primary) text-white"
                    : "background-transparent border-(--moss-border-color)"
                }`}
              >
                <span className="text-lg">🐙</span>
                Github
              </button>
              <button
                type="button"
                onClick={() => setProvider("GITLAB")}
                className={`flex items-center gap-2 rounded-full border px-4 py-1 transition-colors ${
                  provider === "GITLAB"
                    ? "background-(--moss-primary) border-(--moss-primary) text-white"
                    : "background-transparent border-(--moss-border-color)"
                }`}
              >
                <span className="text-lg">🦊</span>
                GitLab
              </button>
            </div>
          </div>

          {/* Flow Section */}
          <div>
            <div className="flex gap-2">
              <span>Flow</span>
              <div className="background-(--moss-border-color) my-auto h-px w-full" />
            </div>
            <p className="text-xs leading-5 text-(--moss-secondary-text)">
              You can switch modes in the workspace at any time and as often as needed.
            </p>
            <div className="pl-5">
              <RadioGroup.Root>
                <RadioGroup.ItemWithLabel
                  label="OAuth 2.0"
                  description="This mode is suitable when your collection is stored in a separate repository or doesn't have a repository at all."
                  value="OAUTH"
                  checked={flow === "OAUTH"}
                  onClick={() => setFlow("OAUTH")}
                />

                <RadioGroup.ItemWithLabel
                  label="PAT"
                  description="This mode is suitable if you want to store the collection in your project's repository or in any other folder you specify."
                  value="PAT"
                  checked={flow === "PAT"}
                  onClick={() => setFlow("PAT")}
                />
              </RadioGroup.Root>
            </div>

            {/* PAT Token Input */}
            {flow === "PAT" && (
              <div className="mt-3 pl-5">
                <div className="flex flex-col gap-1.5">
                  <label className="text-sm text-(--moss-secondary-text)">Token:</label>
                  <InputOutlined
                    ref={tokenInputRef}
                    value={token}
                    onChange={(e) => setToken(e.target.value)}
                    placeholder={`${provider === "GITHUB" ? "github.com" : "gitlab.com"}/moss-foundation/sapic`}
                    className="w-full"
                  />
                </div>
              </div>
            )}
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-between py-0.75">
          <CheckboxWithLabel
            label="Use as default account"
            checked={useAsDefault}
            onCheckedChange={(check) => {
              if (check !== "indeterminate") setUseAsDefault(check);
            }}
          />
          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined type="button" onClick={handleClose} disabled={isSubmitting}>
              Close
            </ButtonNeutralOutlined>
            <ButtonPrimary disabled={isSubmitting || (flow === "PAT" && !token)} type="submit">
              {isSubmitting ? "Connecting..." : "Log In"}
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
