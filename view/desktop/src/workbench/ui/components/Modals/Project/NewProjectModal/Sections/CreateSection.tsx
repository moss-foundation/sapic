import { useEffect, useRef, useState } from "react";

import { useFocusInputOnMount } from "@/hooks";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import { PillTabs } from "@/lib/ui/Tabs/index";
import { VcsProviderSwitcher } from "@/workbench/ui/components/VcsProviderSwitcher";
import { CheckedState } from "@radix-ui/react-checkbox";
import { CreateProjectGitParams } from "@repo/moss-workspace";

import { BranchInput } from "../components/BranchInput";
import { NameInput } from "../components/NameInput";
import { RepositoryInput } from "../components/RepositoryInput";
import { DEFAULT_BRANCH, DEFAULT_NAME, DEFAULT_PROVIDER, DEFAULT_REPOSITORY, DEFAULT_VCS } from "../defaults";

interface CreateSectionProps {
  onValuesUpdate: (values: { name: string; gitParams: CreateProjectGitParams | undefined }) => void;
}

export const CreateSection = ({ onValuesUpdate }: CreateSectionProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const [name, setName] = useState(DEFAULT_NAME);
  const [provider, setProvider] = useState<"github" | "gitlab">(DEFAULT_PROVIDER);
  const [repository, setRepository] = useState(DEFAULT_REPOSITORY);
  const [branch, setBranch] = useState(DEFAULT_BRANCH);
  const [vcs, setVCS] = useState(DEFAULT_VCS);

  useFocusInputOnMount({ inputRef });

  useEffect(() => {
    const deriveGitParams = () => {
      if (!vcs) return undefined;

      if (provider === "github") {
        return {
          gitHub: { repository, branch },
        };
      }

      if (provider === "gitlab") {
        return {
          gitLab: { repository, branch },
        };
      }

      return undefined;
    };

    onValuesUpdate({
      name,
      gitParams: deriveGitParams(),
    });
  }, [name, onValuesUpdate, repository, branch, provider, vcs]);

  const handleSetVCS = (checked: CheckedState) => {
    if (checked === "indeterminate") return;
    setVCS(checked);
  };

  return (
    <div className="flex flex-col gap-3">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-x-2 py-3">
        <NameInput name={name} setName={setName} ref={inputRef} />
      </div>

      <div>
        <div className="flex flex-col gap-2">
          <CheckboxWithLabel checked={vcs} onCheckedChange={handleSetVCS} label="VCS" />
          <span className="text-(--moss-secondary-foreground) text-sm">
            You can switch modes in the workspace at any time and as often as needed.
          </span>
        </div>

        <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pb-2 pl-5 pt-3">
          <VcsProviderSwitcher
            value={provider}
            onValueChange={(value) => setProvider(value as "github" | "gitlab")}
            label="Provider:"
            disabled={!vcs}
            layout="grid"
          >
            <>
              <PillTabs.Content value="github" className="contents">
                <RepositoryInput repository={repository} setRepository={setRepository} disabled={!vcs} />
                <BranchInput branch={branch} setBranch={setBranch} disabled={!vcs} />
              </PillTabs.Content>
              <PillTabs.Content value="gitlab" className="contents">
                <RepositoryInput repository={repository} setRepository={setRepository} disabled={!vcs} />
                <BranchInput branch={branch} setBranch={setBranch} disabled={!vcs} />
              </PillTabs.Content>
            </>
          </VcsProviderSwitcher>
        </div>

        {/* {gitProvider === null && (
          <div className={cn("flex w-full gap-5 py-3 pl-5", !vcs && "opacity-50")}>
            <Button intent="primary"  className="px-3 py-1.5" disabled={!vcs} onClick={handleAddAccount}>
              {provider === "github" ? "Log In via GitHub..." : "Log In via GitLab..."}
            </Button>
            <ButtonNeutralOutlined
              className="px-3 py-1.5"
              disabled={!vcs}
              // onClick={handleAddAccount}
            >
              Log In with Token...
            </Button>
          </div>
        )} */}
      </div>
    </div>
  );
};
