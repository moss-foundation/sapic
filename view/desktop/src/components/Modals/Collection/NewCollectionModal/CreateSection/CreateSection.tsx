import { useEffect, useRef, useState } from "react";

import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import InputOutlined from "@/components/InputOutlined";
import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useFocusInputOnMount } from "@/hooks";
import { useGitProviderStore } from "@/store/gitProvider";
import { cn } from "@/utils";
import { CheckedState } from "@radix-ui/react-checkbox";

import { Provider } from "../ProvidersRadioGroup/ProvidersRadioGroup";
import ProviderTabs from "../ProviderTabs/ProviderTabs";

interface CreateSectionProps {
  onValuesUpdate: (values: {
    name: string;
    repository: string;
    branch: string;
    vcs: boolean;
    provider: Provider | null;
  }) => void;
}

export const CreateSection = ({ onValuesUpdate }: CreateSectionProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState("New Collection");
  const [provider, setProvider] = useState<Provider | null>(null);
  const [repository, setRepository] = useState("github.com/moss-foundation/sapic");
  const [branch, setBranch] = useState("main");
  const [vcs, setVCS] = useState(true);

  useFocusInputOnMount({ inputRef });

  useEffect(() => {
    onValuesUpdate({
      name,
      repository,
      branch,
      vcs,
      provider,
    });
  }, [name, onValuesUpdate, repository, branch, vcs, provider]);

  const handleSetVCS = (checked: CheckedState) => {
    if (checked === "indeterminate") return;

    if (checked === false) {
      setProvider(null);
    }

    setVCS(checked);
  };

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-2">
        <div>Name:</div>
        <InputOutlined
          ref={inputRef}
          value={name}
          className="max-w-72"
          onChange={(e) => setName(e.target.value)}
          pattern={VALID_NAME_PATTERN}
          required
        />
        <p className="col-start-2 max-w-72 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
      </div>

      <div>
        <div className="flex flex-col gap-2">
          <CheckboxWithLabel checked={vcs} onCheckedChange={handleSetVCS} label="VCS" />
          <span className="text-xs text-(--moss-secondary-text)">
            You can switch modes in the workspace at any time and as often as needed.
          </span>
        </div>

        <div className="grid grid-cols-[min-content_1fr] items-center gap-2 py-3 pl-5">
          <ProviderTabs.Root
            value={provider}
            onValueChange={(value) => setProvider(value as Provider)}
            className="contents"
          >
            <ProviderTabs.List className="col-span-2 grid h-min grid-cols-subgrid grid-rows-subgrid">
              <div className={cn(!vcs && "opacity-50")}>Provider:</div>
              <div className="flex gap-2">
                <ProviderTabs.Trigger value="github" label="GitHub" icon="github" disabled={!vcs} />
                <ProviderTabs.Trigger value="gitlab" label="GitLab" icon="gitlab" disabled={!vcs} />
              </div>
            </ProviderTabs.List>

            <ProviderTabs.Content value="github" className="contents">
              <div className="col-span-2 grid grid-cols-subgrid items-center">
                <div className={cn(!vcs && "opacity-50")}>Repository:</div>
                <InputOutlined
                  value={repository}
                  className="max-w-72"
                  onChange={(e) => setRepository(e.target.value)}
                  required
                  disabled={!vcs}
                />
              </div>
              <div className="col-span-2 grid grid-cols-subgrid items-center">
                <div className={cn(!vcs && "opacity-50")}>Branch:</div>
                <InputOutlined
                  value={branch}
                  className="max-w-72"
                  onChange={(e) => setBranch(e.target.value)}
                  pattern={VALID_NAME_PATTERN}
                  required
                  disabled={!vcs}
                />
              </div>
            </ProviderTabs.Content>
            <ProviderTabs.Content value="gitlab" className="contents">
              <div className="col-span-2 grid grid-cols-subgrid items-center">
                <div className={cn(!vcs && "opacity-50")}>Repository:</div>
                <InputOutlined
                  value={repository}
                  className="max-w-72"
                  onChange={(e) => setRepository(e.target.value)}
                  required
                  disabled={!vcs}
                />
              </div>
              <div className="col-span-2 grid grid-cols-subgrid items-center">
                <div className={cn(!vcs && "opacity-50")}>Branch:</div>
                <InputOutlined
                  value={branch}
                  className="max-w-72"
                  onChange={(e) => setBranch(e.target.value)}
                  pattern={VALID_NAME_PATTERN}
                  required
                  disabled={!vcs}
                />
              </div>
            </ProviderTabs.Content>
          </ProviderTabs.Root>
        </div>
      </div>
    </div>
  );
};
