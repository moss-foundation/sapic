import { useRef } from "react";

import { RadioGroup } from "@/components";
import { AccountKind } from "@repo/moss-user";

import { Subheader } from "./Subheader";

interface MethodSectionProps {
  method: "OAUTH" | "PAT";
  setMethod: (method: "OAUTH" | "PAT") => void;
  token: string;
  setToken: (token: string) => void;
  provider: AccountKind;
}

export const MethodSection = ({ method, setMethod, token, setToken, provider }: MethodSectionProps) => {
  const tokenInputRef = useRef<HTMLTextAreaElement>(null);

  return (
    <div>
      <Subheader>
        <span>Method</span>
        <div className="background-(--moss-border-color) my-auto h-px w-full" />
      </Subheader>
      <p className="text-sm leading-5 text-(--moss-secondary-text)">
        Pick the authentication method for connecting your account.
      </p>
      <div className="pl-5">
        <RadioGroup.Root>
          <RadioGroup.ItemWithLabel
            label="OAuth 2.0"
            description="Use your GitHub account directly. Suitable when you don't want to manage tokens manually. Recommended for most users."
            value="OAUTH"
            checked={method === "OAUTH"}
            onClick={() => setMethod("OAUTH")}
            className={method !== "OAUTH" ? "opacity-50" : ""}
          />

          <RadioGroup.ItemWithLabel
            label="PAT"
            description="You can get it in your GitHub settings. The token is stored locally and used only for login."
            value="PAT"
            checked={method === "PAT"}
            onClick={() => setMethod("PAT")}
            className={method !== "PAT" ? "opacity-50" : ""}
          />
        </RadioGroup.Root>
      </div>

      {/* PAT Token Input */}
      {method === "PAT" && (
        <div className="grid grid-cols-[min-content_1fr] items-start gap-x-3 pt-2 pl-10.5">
          <label className="pt-1.5 text-base">Token:</label>
          <textarea
            ref={tokenInputRef}
            value={token}
            onChange={(e) => setToken(e.target.value)}
            placeholder={`${provider === "GITHUB" ? "github" : "gitlab"}_pat_11AJP6K3A0nS9zI77AkyOB_uLU0OUSZu0TRUGo9czDrXzur3kMGpusg9XJpzYaeYYEAKALQUTZ0L3v6q9i`}
            className="h-24.5 w-full resize-none rounded-sm border border-(--moss-border-color) px-2 py-1.5 text-sm placeholder-(--moss-secondary-text) focus:outline-2 focus:outline-(--moss-primary)"
          />
        </div>
      )}
    </div>
  );
};
