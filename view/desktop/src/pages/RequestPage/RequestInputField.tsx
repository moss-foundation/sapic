import React, { useState } from "react";
import { cn } from "@/utils";
import { Icon } from "@/lib/ui";
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
  const [isMethodDropdownOpen, setIsMethodDropdownOpen] = useState(false);

  const handleSend = () => {
    onSend?.(method, url);
  };

  const handleTemplateChange = (value: string) => {
    setUrl(value);
  };

  return (
    <div className={cn("relative flex w-full items-center rounded-sm border border-gray-200 bg-white", className)}>
      {/* HTTP Method Dropdown - positioned inside the input field on the left */}
      <div className="relative flex-shrink-0">
        <button
          onClick={() => setIsMethodDropdownOpen(!isMethodDropdownOpen)}
          className={cn(
            "flex items-center gap-1 rounded-l-sm px-3 py-2 text-sm font-medium transition-colors",
            "bg-gray-100 text-orange-600 hover:bg-gray-200",
            "focus:ring-2 focus:ring-orange-500 focus:ring-offset-1 focus:outline-none",
            "border-r border-gray-200"
          )}
        >
          <span>{method}</span>
          <Icon icon="ChevronDown" className="h-3 w-3" />
        </button>

        {isMethodDropdownOpen && (
          <>
            {/* Backdrop */}
            <div className="fixed inset-0 z-10" onClick={() => setIsMethodDropdownOpen(false)} />

            {/* Dropdown Menu */}
            <div className="absolute top-full left-0 z-20 mt-1 min-w-[120px] rounded-sm border border-gray-200 bg-white shadow-lg">
              {HTTP_METHODS.map((httpMethod) => (
                <button
                  key={httpMethod}
                  onClick={() => {
                    setMethod(httpMethod);
                    setIsMethodDropdownOpen(false);
                  }}
                  className={cn(
                    "w-full px-3 py-2 text-left text-sm transition-colors hover:bg-gray-50",
                    method === httpMethod && "bg-orange-50 font-medium text-orange-600"
                  )}
                >
                  {httpMethod}
                </button>
              ))}
            </div>
          </>
        )}
      </div>

      {/* URL Input Field - takes up the middle space */}
      <div className="flex-1">
        <InputTemplating
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          onTemplateChange={handleTemplateChange}
          className="rounded-none border-0 bg-transparent shadow-none"
          size="md"
          placeholder="Enter URL..."
        />
      </div>

      {/* Send Button - positioned inside the input field on the right */}
      <button
        onClick={handleSend}
        className={cn(
          "flex-shrink-0 rounded-r-sm bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700",
          "transition-colors focus:ring-2 focus:ring-blue-500 focus:ring-offset-1 focus:outline-none",
          "flex items-center gap-2 border-l border-gray-200"
        )}
      >
        <Icon icon="Send" className="h-4 w-4" />
        Send
      </button>
    </div>
  );
};
