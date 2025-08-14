import { useRef, useState } from "react";

import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import InputOutlined from "@/components/InputOutlined";
import { VALID_NAME_PATTERN } from "@/constants/validation";

export const CreateSection = () => {
  const inputRef = useRef<HTMLInputElement>(null);
  const [name, setName] = useState("");
  const [provider, setProvider] = useState("");
  const [repository, setRepository] = useState("");
  const [branch, setBranch] = useState("");
  const [vcs, setVcs] = useState(true);

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-3">
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
          <CheckboxWithLabel checked={vcs} onCheckedChange={() => setVcs(!vcs)} label="VCS" />
          <span className="text-xs text-(--moss-secondary-text)">
            You can switch modes in the workspace at any time and as often as needed.
          </span>
        </div>

        <div className="grid grid-cols-[min-content_1fr] items-center gap-3 py-3 pl-5">
          <div className="col-span-2 grid grid-cols-subgrid items-center">
            <div>Provider:</div>
            <InputOutlined
              ref={inputRef}
              value={provider}
              className="max-w-72"
              onChange={(e) => setProvider(e.target.value)}
              pattern={VALID_NAME_PATTERN}
              required
            />
          </div>
          <div className="col-span-2 grid grid-cols-subgrid">
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
          <div className="col-span-2 grid grid-cols-subgrid">
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
    </div>
  );
};
