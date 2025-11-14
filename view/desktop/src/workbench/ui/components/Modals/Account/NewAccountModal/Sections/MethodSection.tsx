import { useRef } from "react";

import { Link } from "@/lib/ui";
import { RadioGroup } from "@/workbench/ui/components";
import { AccountKind } from "@repo/moss-user";

import { getPatPlaceholder, getProviderName, getProviderSettingsUrl } from "../../accountUtils";
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

  const providerName = getProviderName(provider);
  const settingsUrl = getProviderSettingsUrl(provider);

  return (
    <div>
      <Subheader>
        <span>Method</span>
        <div className="background-(--moss-border) my-auto h-px w-full" />
      </Subheader>
      <p className="text-(--moss-secondary-foreground) text-sm leading-5">
        Pick the authentication method for connecting your account.
      </p>
      <div className="mt-2 pl-5">
        <RadioGroup.Root>
          <RadioGroup.ItemWithLabel
            label="OAuth 2.0"
            description={`Use your ${providerName} account directly. Suitable when you don't want to manage tokens manually. Recommended for most users.`}
            value="OAUTH"
            checked={method === "OAUTH"}
            onClick={() => setMethod("OAUTH")}
            className={method !== "OAUTH" ? "opacity-50" : ""}
          />

          <RadioGroup.ItemWithLabel
            label="PAT"
            description={
              <span>
                You can get it in your{" "}
                <Link href={settingsUrl} target="_blank" rel="noopener noreferrer">
                  {providerName}
                </Link>{" "}
                settings. The token is stored locally and used only for login.
              </span>
            }
            value="PAT"
            checked={method === "PAT"}
            onClick={() => setMethod("PAT")}
            className={method !== "PAT" ? "opacity-50" : ""}
          />
        </RadioGroup.Root>
      </div>

      {/* PAT Token Input */}
      {method === "PAT" && (
        <div className="pl-10.5 grid grid-cols-[min-content_1fr] items-start gap-x-3 pt-3.5">
          <label className="pt-1.5 text-base">Token:</label>
          <textarea
            ref={tokenInputRef}
            value={token}
            onChange={(e) => setToken(e.target.value)}
            placeholder={getPatPlaceholder(provider)}
            className="h-24.5 border-(--moss-border) placeholder-(--moss-secondary-foreground) focus:outline-(--moss-primary) w-full resize-none rounded-sm border px-2 py-1.5 text-sm focus:outline-2"
          />
        </div>
      )}
    </div>
  );
};
