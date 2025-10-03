import { useState } from "react";

import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { Button, Icon } from "@/lib/ui";
import { Input } from "@/lib/ui/Input";
import MossSelect from "@/lib/ui/MossSelect";
import { invoke } from "@tauri-apps/api/core";
import { AddAccountParams, UpdateProfileInput } from "@repo/moss-app";
import { AccountInfo, ProfileInfo } from "@repo/moss-user";

import { ProfilePageProps } from "../../../ProfilePage";

interface AccountsTabProps extends IDockviewPanelProps<ProfilePageProps> {
  profile: ProfileInfo;
}

export const AccountsTab = ({ profile }: AccountsTabProps) => {
  const [isAddingAccount, setIsAddingAccount] = useState(false);
  const [accountForm, setAccountForm] = useState<AddAccountParams>({
    host: "github.com",
    label: "",
    kind: "GITHUB",
    pat: "",
  });
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleAddAccount = async () => {
    try {
      setIsSubmitting(true);
      const input: UpdateProfileInput = {
        accountsToAdd: [accountForm],
        accountsToRemove: [],
      };
      await invoke("update_profile", { input });
      console.log("Account added successfully");

      // Reset form
      setAccountForm({
        host: "github.com",
        label: "",
        kind: "GITHUB",
        pat: "",
      });
      setIsAddingAccount(false);

      // Refresh the page data
      window.location.reload();
    } catch (error) {
      console.error("Error adding account:", error);
      alert(`Failed to add account: ${error}`);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleRemoveAccount = async (accountId: string) => {
    if (!confirm("Are you sure you want to remove this account?")) {
      return;
    }

    try {
      setIsSubmitting(true);
      const input: UpdateProfileInput = {
        accountsToAdd: [],
        accountsToRemove: [accountId],
      };
      await invoke("update_profile", { input });
      console.log("Account removed successfully");

      // Refresh the page data
      window.location.reload();
    } catch (error) {
      console.error("Error removing account:", error);
      alert(`Failed to remove account: ${error}`);
    } finally {
      setIsSubmitting(false);
    }
  };

  const getProviderIcon = (kind: string) => {
    switch (kind) {
      case "GITHUB":
        return "VCS";
      case "GITLAB":
        return "VCS";
      default:
        return "Placeholder";
    }
  };

  return (
    <div className="flex flex-col gap-6">
      <section>
        <div className="mb-3 flex items-center justify-between">
          <h3 className="text-lg font-medium">Connected Accounts</h3>
          {!isAddingAccount && (
            <Button
              onClick={() => setIsAddingAccount(true)}
              className="background-(--moss-primary) hover:background-(--moss-primary-hover) flex items-center gap-2 px-3 py-1.5 text-sm text-(--moss-primary-text-inverse)"
            >
              <Icon icon="Plus" className="size-4" />
              Add Account
            </Button>
          )}
        </div>

        {isAddingAccount && (
          <div className="mb-4 rounded-md border border-(--moss-border-color) p-4">
            <h4 className="mb-3 text-base font-medium">Add New Account</h4>
            <div className="flex flex-col gap-3">
              <div className="flex flex-col gap-1.5">
                <label className="text-sm text-(--moss-secondary-text)">Host</label>
                <Input
                  type="text"
                  value={accountForm.host}
                  onChange={(e) => setAccountForm((prev) => ({ ...prev, host: e.target.value }))}
                  placeholder="github.com"
                  className="background-(--moss-secondary-background) border border-(--moss-border-color) px-3 py-2"
                />
              </div>

              <div className="flex flex-col gap-1.5">
                <label className="text-sm text-(--moss-secondary-text)">Label (optional)</label>
                <Input
                  type="text"
                  value={accountForm.label}
                  onChange={(e) => setAccountForm((prev) => ({ ...prev, label: e.target.value }))}
                  placeholder="My GitHub Account"
                  className="background-(--moss-secondary-background) border border-(--moss-border-color) px-3 py-2"
                />
              </div>

              <div className="flex flex-col gap-1.5">
                <label className="text-sm text-(--moss-secondary-text)">Provider</label>
                <MossSelect.Root
                  value={accountForm.kind}
                  onValueChange={(value) => setAccountForm((prev) => ({ ...prev, kind: value as "GITHUB" | "GITLAB" }))}
                >
                  <MossSelect.Trigger placeholder="Select provider" />
                  <MossSelect.Content>
                    <MossSelect.Item value="GITHUB">GitHub</MossSelect.Item>
                    <MossSelect.Item value="GITLAB">GitLab</MossSelect.Item>
                  </MossSelect.Content>
                </MossSelect.Root>
              </div>

              <div className="flex flex-col gap-1.5">
                <label className="text-sm text-(--moss-secondary-text)">Personal Access Token (optional)</label>
                <Input
                  type="password"
                  value={accountForm.pat}
                  onChange={(e) => {
                    const { pat: _pat, ...rest } = accountForm;
                    if (e.target.value === "") {
                      setAccountForm(rest);
                    } else {
                      setAccountForm({ ...rest, pat: e.target.value });
                    }
                  }}
                  placeholder="Leave empty to use OAuth"
                  className="background-(--moss-secondary-background) border border-(--moss-border-color) px-3 py-2"
                />
              </div>

              <div className="flex gap-2">
                <Button
                  onClick={handleAddAccount}
                  disabled={isSubmitting || !accountForm.host}
                  loading={isSubmitting}
                  className="background-(--moss-primary) hover:background-(--moss-primary-hover) px-4 py-2 text-(--moss-primary-text-inverse)"
                >
                  Add Account
                </Button>
                <Button
                  onClick={() => {
                    setIsAddingAccount(false);
                    setAccountForm({
                      host: "github.com",
                      label: "",
                      kind: "GITHUB",
                      pat: "",
                    });
                  }}
                  disabled={isSubmitting}
                  className="border border-(--moss-border-color) px-4 py-2"
                >
                  Cancel
                </Button>
              </div>
            </div>
          </div>
        )}

        <div className="flex flex-col gap-3">
          {profile.accounts.length === 0 ? (
            <div className="rounded-md border border-(--moss-border-color) p-8 text-center">
              <Icon icon="Person" className="mx-auto mb-2 size-12 opacity-30" />
              <p className="text-(--moss-secondary-text)">No accounts connected yet</p>
              <p className="mt-1 text-sm text-(--moss-secondary-text)">Add an account to get started</p>
            </div>
          ) : (
            profile.accounts.map((account: AccountInfo) => (
              <div
                key={account.id}
                className="flex items-center justify-between rounded-md border border-(--moss-border-color) p-4"
              >
                <div className="flex items-center gap-3">
                  <Icon icon={getProviderIcon(account.kind)} className="size-6" />
                  <div className="flex flex-col">
                    <div className="flex items-center gap-2">
                      <span className="font-medium">{account.username}</span>
                      <span className="text-xs text-(--moss-secondary-text)">@{account.host}</span>
                    </div>
                    <span className="text-sm text-(--moss-secondary-text)">{account.kind}</span>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <Button
                    onClick={() => console.log("Edit details for", account.id)}
                    className="border border-(--moss-border-color) px-3 py-1.5 text-sm"
                  >
                    Edit details
                  </Button>
                  <Button
                    onClick={() => handleRemoveAccount(account.id)}
                    disabled={isSubmitting}
                    className="hover:background-(--moss-error) border border-(--moss-error) px-3 py-1.5 text-sm text-(--moss-error) hover:text-(--moss-primary-text-inverse)"
                  >
                    Revoke
                  </Button>
                </div>
              </div>
            ))
          )}
        </div>
      </section>
    </div>
  );
};
