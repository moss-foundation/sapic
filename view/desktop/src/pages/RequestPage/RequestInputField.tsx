import React, { useState, useRef } from "react";
import { cn } from "@/utils";
import { Icon, Scrollbar } from "@/lib/ui";
import { ActionMenu, Divider, ButtonPrimary } from "@/components";
import InputTemplating from "@/components/InputTemplating";

interface RequestInputFieldProps {
  className?: string;
  initialMethod?: string;
  initialUrl?: string;
  onSend?: (method: string, url: string) => void;
  onUrlChange?: (url: string) => void;
  onMethodChange?: (method: string) => void;
}

const HTTP_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

export const RequestInputField: React.FC<RequestInputFieldProps> = React.memo(
  ({
    className,
    initialMethod = "POST",
    initialUrl = "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?queryParam={{queryValue}}",
    onSend,
    onUrlChange,
    onMethodChange,
  }) => {
    const [method, setMethod] = useState(initialMethod);
    const [url, setUrl] = useState(initialUrl);
    const lastExternalUrlRef = useRef(initialUrl);
    const isUserTypingRef = useRef(false);

    // Sync method changes
    React.useEffect(() => {
      if (initialMethod !== method) {
        setMethod(initialMethod);
      }
    }, [initialMethod, method]);

    // Only sync URL from external source when user is not actively typing
    React.useEffect(() => {
      if (initialUrl !== lastExternalUrlRef.current && !isUserTypingRef.current) {
        setUrl(initialUrl);
        lastExternalUrlRef.current = initialUrl;
      }
    }, [initialUrl]);

    const handleSend = () => {
      onSend?.(method, url);
    };

    // Debounced URL change handler to prevent rapid updates
    const debouncedOnUrlChange = React.useCallback(
      (() => {
        let timeoutId: NodeJS.Timeout;
        return (value: string) => {
          clearTimeout(timeoutId);
          timeoutId = setTimeout(() => {
            isUserTypingRef.current = false;
            onUrlChange?.(value);
          }, 150); // 150ms debounce
        };
      })(),
      [onUrlChange]
    );

    // Optimized change handlers with stable references
    const handleTemplateChange = React.useCallback(
      (value: string) => {
        isUserTypingRef.current = true;
        setUrl(value);
        debouncedOnUrlChange(value);
      },
      [debouncedOnUrlChange]
    );

    const handleMethodChange = React.useCallback(
      (newMethod: string) => {
        setMethod(newMethod);
        onMethodChange?.(newMethod);
      },
      [onMethodChange]
    );

    return (
      <div className={cn("flex w-full min-w-0 items-center rounded border border-(--moss-border-color)", className)}>
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
                  onClick={() => handleMethodChange(httpMethod)}
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
        <div className="m-[-4px] flex h-10 flex-shrink-0 items-center border-t border-b border-transparent">
          <Divider height="medium" className="mx-0" />
        </div>
        {/* Center - URL Input Field */}
        <div className="min-w-0 flex-1">
          <Scrollbar
            className="h-10 overflow-x-auto overflow-y-hidden rounded-none border-r-0 border-l-0 border-transparent"
            options={{
              scrollbars: {
                autoHide: "move",
              },
            }}
          >
            <div className="min-w-max">
              <InputTemplating
                value={url}
                onTemplateChange={handleTemplateChange}
                className="h-10 w-auto min-w-max rounded-none border-r-0 border-l-0 border-transparent whitespace-nowrap [&>div]:overflow-visible"
                size="md"
                placeholder="Enter URL..."
                highlightColonVariables={true}
              />
            </div>
          </Scrollbar>
        </div>

        {/* Right Side - Send Button */}
        <div className="relative z-10 flex h-10 flex-shrink-0 items-center rounded-md rounded-l-none border border-l-0 border-transparent p-1 focus-within:outline-2 focus-within:-outline-offset-1 focus-within:outline-(--moss-primary)">
          <ButtonPrimary onClick={handleSend}>Send</ButtonPrimary>
        </div>
      </div>
    );
  }
);
