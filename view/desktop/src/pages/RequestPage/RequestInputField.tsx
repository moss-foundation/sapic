import React, { useState } from "react";
import { cn } from "@/utils";
import { Icon } from "@/lib/ui";
import { ActionMenu } from "@/components";
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
  initialUrl = "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns",
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
    <div className={cn("flex w-full items-center", className)}>
      {/* Left Side - HTTP Method Dropdown */}
      <div className="relative flex-shrink-0">
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <button
              className={cn(
                "flex items-center gap-1 rounded-sm px-3 py-2 text-sm font-medium transition-colors",
                "background-(--moss-primary-background) text-orange-600",
                "focus:ring-2 focus:ring-orange-500 focus:ring-offset-1 focus:outline-none",
                "border border-gray-200"
              )}
            >
              <span>{method}</span>
              <Icon icon="ChevronDown" className="h-3 w-3" />
            </button>
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            {HTTP_METHODS.map((httpMethod) => (
              <ActionMenu.Item
                key={httpMethod}
                onClick={() => setMethod(httpMethod)}
                className={cn(
                  method === httpMethod && "background-(--moss-secondary-background-hover) font-medium text-orange-600"
                )}
              >
                {httpMethod}
              </ActionMenu.Item>
            ))}
          </ActionMenu.Content>
        </ActionMenu.Root>
      </div>

      {/* Center - URL Input Field */}
      <div className="flex-1">
        <InputTemplating
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          onTemplateChange={handleTemplateChange}
          className="w-full rounded-none border-r-0 border-l-0"
          size="md"
          placeholder="Enter URL..."
        />
      </div>

      {/* Right Side - Send Button */}
      <button
        onClick={handleSend}
        className={cn(
          "flex-shrink-0 rounded-sm bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700",
          "transition-colors focus:ring-2 focus:ring-blue-500 focus:ring-offset-1 focus:outline-none",
          "flex items-center gap-2"
        )}
      >
        <Icon icon="Send" className="h-4 w-4" />
        Send
      </button>
    </div>
  );
};
