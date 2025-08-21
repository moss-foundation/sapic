import { memo, useCallback, useEffect, useRef, useState } from "react";

import { ActionMenu, ButtonPrimary } from "@/components";
import InputTemplating from "@/components/InputTemplating";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { areUrlsEquivalent } from "./utils/urlParser";

interface RequestInputFieldProps {
  className?: string;
  initialMethod?: string;
  initialUrl?: string;
  onSend?: (method: string, url: string) => void;
  onUrlChange?: (url: string) => void;
  onMethodChange?: (method: string) => void;
}

const HTTP_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

export const RequestInputField = memo(
  ({
    className,
    initialMethod = "POST",
    initialUrl = "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?sort={{sortValue}}&limit=2",
    onSend,
    onUrlChange,
    onMethodChange,
  }: RequestInputFieldProps) => {
    const [method, setMethod] = useState(initialMethod);
    const [url, setUrl] = useState(initialUrl);
    const lastExternalUrlRef = useRef(initialUrl);
    const isUserTypingRef = useRef(false);
    const lastSentUrlRef = useRef("");

    // Sync method changes
    useEffect(() => {
      if (initialMethod !== method) {
        setMethod(initialMethod);
      }
    }, [initialMethod, method]);

    // Only sync URL from external source when user is not actively typing
    useEffect(() => {
      // Use normalized comparison to prevent unnecessary updates
      if (
        !areUrlsEquivalent(initialUrl, lastExternalUrlRef.current) &&
        !isUserTypingRef.current &&
        !areUrlsEquivalent(initialUrl, lastSentUrlRef.current)
      ) {
        setUrl(initialUrl);
        lastExternalUrlRef.current = initialUrl;
      }
    }, [initialUrl]);

    const handleSend = () => {
      onSend?.(method, url);
    };

    // Debounced URL change handler with normalized comparison
    const debouncedOnUrlChange = useCallback(
      (() => {
        let timeoutId: NodeJS.Timeout;
        return (value: string) => {
          clearTimeout(timeoutId);
          timeoutId = setTimeout(() => {
            // Only call onUrlChange if the URL has actually changed (normalized comparison)
            if (!areUrlsEquivalent(value, lastSentUrlRef.current)) {
              lastSentUrlRef.current = value;
              onUrlChange?.(value);
            }
            isUserTypingRef.current = false;
          }, 150); // 150ms debounce
        };
      })(),
      [onUrlChange]
    );

    // Optimized change handlers with stable references
    const handleTemplateChange = useCallback(
      (value: string) => {
        isUserTypingRef.current = true;
        setUrl(value);
        debouncedOnUrlChange(value);
      },
      [debouncedOnUrlChange]
    );

    const handleMethodChange = useCallback(
      (newMethod: string) => {
        setMethod(newMethod);
        onMethodChange?.(newMethod);
      },
      [onMethodChange]
    );

    return (
      <div
        className={cn("relative flex min-w-0 items-start rounded-md border border-(--moss-border-color)", className)}
      >
        {/* Left Side - HTTP Method Dropdown */}
        <div className="relative">
          <ActionMenu.Root>
            <ActionMenu.Trigger asChild>
              <button
                className={cn(
                  "flex items-center justify-between",
                  "gap-0.5 px-2.5 py-3 pr-1 pl-3",
                  "transition-colors",
                  "rounded-md rounded-r-none",
                  "cursor-pointer font-bold",
                  "background-(--moss-primary-background) text-(--moss-requestpage-text)",
                  "data-[state=open]:outline-2 data-[state=open]:-outline-offset-1 data-[state=open]:outline-(--moss-primary)"
                )}
              >
                <span>{method}</span>
                <Icon icon="ChevronDown" />
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

        {/* Center - URL Input Field */}
        <div className="relative z-20 min-w-0 flex-1">
          <InputTemplating
            value={url}
            onTemplateChange={handleTemplateChange}
            className="w-full rounded-none border-r-0 border-l-0 border-transparent"
            size="md"
            placeholder="Enter URL..."
            highlightColonVariables={true}
          />
        </div>

        {/* Right Side - Send Button */}
        <div className="relative z-30 flex min-h-8 flex-shrink-0 items-center rounded-md rounded-l-none border border-l-0 border-transparent p-1">
          <ButtonPrimary onClick={handleSend}>Send</ButtonPrimary>
        </div>
      </div>
    );
  }
);
