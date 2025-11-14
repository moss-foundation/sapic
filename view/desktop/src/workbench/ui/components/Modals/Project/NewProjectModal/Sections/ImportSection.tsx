import { useEffect, useState } from "react";

import Input from "@/lib/ui/Input";
import { PillTabs } from "@/lib/ui/Tabs/index";
import { useGitProviderStore } from "@/store/gitProvider";
import { VcsProviderSwitcher } from "@/workbench/ui/components/VcsProviderSwitcher";
import { ImportProjectSource } from "@repo/moss-workspace";

import { BranchInput } from "../components/BranchInput";
import { NameInput } from "../components/NameInput";
import { RepositoryInput } from "../components/RepositoryInput";
import { DEFAULT_BRANCH, DEFAULT_NAME, DEFAULT_PROVIDER, DEFAULT_REPOSITORY } from "../defaults";
import { Subheader } from "./Subheader";

interface ImportSectionProps {
  onValuesUpdate: (values: { name: string; importParams: ImportProjectSource | undefined }) => void;
}

export const ImportSection = ({ onValuesUpdate }: ImportSectionProps) => {
  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState(DEFAULT_NAME);
  //TODO repository expects input like this: https://github.com/brutusyhy/test-empty-collection.git.
  const [repository, setRepository] = useState(DEFAULT_REPOSITORY);
  const [branch, setBranch] = useState(DEFAULT_BRANCH);
  const [provider, setProvider] = useState<"github" | "gitlab">(DEFAULT_PROVIDER);
  const [accountId, setAccountId] = useState("");

  useEffect(() => {
    const deriveGitParams = () => {
      if (provider === "github") {
        return {
          gitHub: { accountId, repository, branch },
        };
      }

      if (provider === "gitlab") {
        return {
          gitLab: { accountId, repository, branch },
        };
      }

      return undefined;
    };

    onValuesUpdate({
      name,
      importParams: deriveGitParams(),
    });
  }, [name, onValuesUpdate, repository, branch, provider, accountId]);

  const handleAddAccount = () => {};

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pb-2">
        <VcsProviderSwitcher
          value={provider}
          onValueChange={(value) => setProvider(value as "github" | "gitlab")}
          label="From:"
          layout="grid"
        >
          <PillTabs.Content value="github" className="contents">
            <NameInput name={name} setName={setName} />
          </PillTabs.Content>
          <PillTabs.Content value="gitlab" className="contents">
            <NameInput name={name} setName={setName} />
          </PillTabs.Content>
        </VcsProviderSwitcher>
      </div>

      <div>
        <Subheader>
          <span>Git</span>
          <div className="background-(--moss-border) my-auto h-px w-full" />
          {gitProvider === null && (
            <button
              className="text-(--moss-primary) cursor-pointer whitespace-nowrap hover:underline"
              onClick={handleAddAccount}
              type="button"
            >
              Log In via GitHub
            </button>
          )}
        </Subheader>

        <span className="text-(--moss-secondary-foreground) text-sm">
          You can switch modes in the workspace at any time and as often as needed.
        </span>

        <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pb-2 pl-5 pt-3">
          <RepositoryInput repository={repository} setRepository={setRepository} />

          <BranchInput branch={branch} setBranch={setBranch} />

          <div className="col-span-2 grid grid-cols-subgrid items-center">
            <div>Account:</div>
            <Input
              className="max-w-72"
              value={accountId}
              onChange={(e) => setAccountId(e.target.value)}
              placeholder="Account"
              required
              intent="outlined"
            />
          </div>
        </div>
      </div>
    </div>
  );
};
