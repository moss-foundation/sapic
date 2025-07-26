import React, { useState } from "react";
import { cn } from "@/utils";
import { Icon } from "@/lib/ui";
import { ActionMenu, Divider, ButtonPrimary } from "@/components";
import InputTemplating from "@/components/InputTemplating";

interface RequestInputFieldProps {
  className?: string;
  initialMethod?: string;
  initialUrl?: string;
  onSend?: (method: string, url: string) => void;
}

const HTTP_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

export const RequestInputField: React.FC<RequestInputFieldProps> = ({
  className,
  initialMethod = "POST",
  initialUrl = "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?queryParam={{queryValue}}",
  onSend,
}) => {
  const [method, setMethod] = useState(initialMethod);
  const [url, setUrl] = useState(initialUrl);

  const handleSend = () => {
    onSend?.(method, url);
  };

  const handleTemplateChange = (value: string) => {
    setUrl(value);
  };

  return (
    <div
      className={cn("flex w-full items-center rounded-md border-1 border-(--moss-requestpage-border-color)", className)}
    >
      {/* Left Side - HTTP Method Dropdown */}
      <div className="relative flex-shrink-0">
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <button
              className={cn(
                "flex items-center justify-between rounded-md rounded-r-none px-2.5 py-2 text-base font-bold transition-colors",
                "background-(--moss-primary-background) text-(--moss-requestpage-text)",
                "focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-(--moss-primary)",
                "data-[state=open]:outline-2 data-[state=open]:-outline-offset-1 data-[state=open]:outline-(--moss-primary)",
                "border border-r-0 border-transparent",
                "h-10 w-24"
              )}
            >
              <span>{method}</span>
              <Icon icon="ChevronDown" className="h-3 w-3 cursor-pointer" />
            </button>
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            {HTTP_METHODS.map((httpMethod) => (
              <ActionMenu.Item
                key={httpMethod}
                onClick={() => setMethod(httpMethod)}
                className={cn(
                  method === httpMethod &&
                    "background-(--moss-secondary-background-hover) font-medium text-(--moss-requestpage-text)"
                )}
              >
                {httpMethod}
              </ActionMenu.Item>
            ))}
          </ActionMenu.Content>
        </ActionMenu.Root>
      </div>
      {/* Divider between HTTP Method and URL Input */}
      <div className="m-[-4px] flex h-10 items-center border-t border-b border-transparent">
        <Divider height="medium" className="mx-0" />
      </div>
      {/* Center - URL Input Field */}
      <div className="flex-1">
        <InputTemplating
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          onTemplateChange={handleTemplateChange}
          className="h-10 w-full rounded-none border-r-0 border-l-0 border-transparent"
          size="md"
          placeholder="Enter URL..."
        />
      </div>

      {/* Right Side - Send Button */}
      <div className="relative z-10 flex h-10 items-center rounded-md rounded-l-none border border-l-0 border-transparent p-1 focus-within:outline-2 focus-within:-outline-offset-1 focus-within:outline-(--moss-primary)">
        <ButtonPrimary onClick={handleSend}>Send</ButtonPrimary>
      </div>
    </div>
  );
};
