import { useState } from "react";

import { Notification } from "@/lib/ui";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const NotificationsSection = () => {
  const [showCustom, setShowCustom] = useState(false);

  return (
    <KitchenSinkSection
      header="Notification Components"
      description="Notification cards with different variants, icons, and action buttons for various user feedback scenarios."
    >
      {/* Info Notification */}
      <div>
        <h3 className="mb-3 text-lg font-semibold text-gray-700 dark:text-gray-300">Info Notification</h3>
        <Notification
          variant="info"
          title="JDK 18 required"
          description="You need to install JDK 18 in order to run this project."
          buttonText="Install JDK 18"
          linkText="Remind me later"
          onButtonClick={() => alert("Install JDK 18 clicked!")}
          onLinkClick={() => alert("Remind me later clicked!")}
        />
      </div>

      {/* Warning Notification */}
      <div>
        <h3 className="mb-3 text-lg font-semibold text-gray-700 dark:text-gray-300">Warning Notification</h3>
        <Notification
          variant="warning"
          title="Low memory"
          description="The IDE is running low on memory and this might affect performance. Please consider increasing the heap size."
          buttonText="Analyze memory use"
          onButtonClick={() => alert("Analyze memory use clicked!")}
        />
      </div>

      {/* Error Notification */}
      <div>
        <h3 className="mb-3 text-lg font-semibold text-gray-700 dark:text-gray-300">Error Notification</h3>
        <Notification
          variant="error"
          title="Build failed"
          description="The compilation process encountered errors. Please check your code for syntax issues."
          buttonText="View errors"
          linkText="Ignore for now"
          onButtonClick={() => alert("View errors clicked!")}
          onLinkClick={() => alert("Ignore for now clicked!")}
        />
      </div>

      {/* Minimal Notifications */}
      <div>
        <h3 className="mb-3 text-lg font-semibold text-gray-700 dark:text-gray-300">Minimal Variants</h3>
        <div className="space-y-3">
          <Notification variant="info" title="File saved successfully" />
          <Notification variant="warning" description="This action cannot be undone." />
          <Notification
            variant="error"
            title="Connection failed"
            linkText="Retry connection"
            onLinkClick={() => alert("Retry connection clicked!")}
          />
        </div>
      </div>

      {/* Custom Icon */}
      <div>
        <h3 className="mb-3 text-lg font-semibold text-gray-700 dark:text-gray-300">Custom Icon</h3>
        <Notification
          variant="info"
          icon="Bell"
          title="2,662 files updated in 844 commits"
          linkText="View commits"
          onLinkClick={() => alert("View commits clicked!")}
        />
      </div>

      {/* No Icon */}
      <div>
        <h3 className="mb-3 text-lg font-semibold text-gray-700 dark:text-gray-300">No Icon</h3>
        <Notification
          variant="warning"
          icon={null}
          title="Update available"
          description="A new version of the application is available for download."
          buttonText="Download now"
          onButtonClick={() => alert("Download now clicked!")}
        />
      </div>

      {/* Custom Content */}
      <div>
        <h3 className="mb-3 text-lg font-semibold text-gray-700 dark:text-gray-300">Custom Content</h3>
        <Notification variant="info">
          <div className="flex items-start gap-3">
            <div className="mt-0.5 flex-shrink-0">
              <div className="flex size-5 items-center justify-center rounded-full bg-blue-400">
                <span className="text-xs font-bold text-blue-900">!</span>
              </div>
            </div>
            <div className="flex-1">
              <div className="mb-1 font-semibold text-blue-100">Custom Layout</div>
              <div className="mb-3 text-sm text-blue-200">
                This notification uses custom children to demonstrate the flexibility of the component.
              </div>
              <button
                onClick={() => setShowCustom(!showCustom)}
                className="text-sm text-blue-300 underline-offset-4 hover:text-blue-200 hover:underline"
              >
                {showCustom ? "Hide details" : "Show details"}
              </button>
              {showCustom && (
                <div className="mt-2 rounded bg-blue-900/50 p-2 text-sm text-blue-200">
                  Additional content can be shown here when needed.
                </div>
              )}
            </div>
          </div>
        </Notification>
      </div>
    </KitchenSinkSection>
  );
};
