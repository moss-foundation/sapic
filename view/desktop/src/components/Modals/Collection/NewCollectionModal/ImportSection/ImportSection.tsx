import { useRef, useState } from "react";

import InputOutlined from "@/components/InputOutlined";
import { VALID_NAME_PATTERN } from "@/constants/validation";

import { Provider, Providers, ProvidersRadioGroup } from "../ProvidersRadioGroup/ProvidersRadioGroup";

export const ImportSection = () => {
  const inputRef = useRef<HTMLInputElement>(null);

  const [name, setName] = useState("");
  const [repository, setRepository] = useState("");
  const [branch, setBranch] = useState("");
  const [provider, setProvider] = useState<Provider | null>("github");

  const providers: Providers = [
    { value: "github", label: "GitHub", icon: "github" },
    { value: "gitlab", label: "GitLab", icon: "gitlab" },
    { value: "postman", label: "Postman", icon: "postman" },
    { value: "insomnia", label: "Insomnia", icon: "insomnia" },
  ];

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-3">
        <div>From:</div>
        <ProvidersRadioGroup selected={provider} setSelected={setProvider} providers={providers} />

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

      <div className="flex justify-between gap-2">
        <span>Git</span>
        <div className="background-(--moss-border-color) my-auto h-px w-full" />
        <a className="cursor-pointer whitespace-nowrap text-(--moss-primary) hover:underline" href="">
          Login In via GitHub
        </a>
      </div>

      <span className="text-xs text-(--moss-secondary-text)">
        You can switch modes in the workspace at any time and as often as needed.
      </span>

      <div className="grid grid-cols-[min-content_1fr] items-center gap-3 py-3 pl-5">
        <div className="col-span-2 grid grid-cols-subgrid items-center">
          <div>Repository:</div>
          <InputOutlined
            ref={inputRef}
            value={repository}
            className="max-w-72"
            onChange={(e) => setRepository(e.target.value)}
            pattern={VALID_NAME_PATTERN}
            required
          />
        </div>

        <div className="col-span-2 grid grid-cols-subgrid items-center">
          <div>Branch:</div>
          <InputOutlined
            ref={inputRef}
            value={branch}
            className="max-w-72"
            onChange={(e) => setBranch(e.target.value)}
            pattern={VALID_NAME_PATTERN}
            required
          />
        </div>
      </div>
    </div>
  );
};
