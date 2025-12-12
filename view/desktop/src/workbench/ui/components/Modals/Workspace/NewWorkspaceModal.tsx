import { useRef, useState } from "react";

import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useFocusInputOnMount } from "@/hooks";
import { useCreateWorkspace } from "@/hooks/workbench/useCreateWorkspace";
import { useOpenWorkspace } from "@/hooks/workbench/useOpenWorkspace";
import { Button } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import Input from "@/lib/ui/Input";
import { OpenInTargetEnum } from "@/main/types";
import { useWelcomeOpenWorkspace } from "@/welcome/adapters/tanstackQuery/workspace";
import { useWelcomeCreateWorkspace } from "@/welcome/adapters/tanstackQuery/workspace/useWelcomeCreateWorkspace";
import { RadioGroup } from "@/workbench/ui/components";
import { ModalForm } from "@/workbench/ui/components/ModalForm";
import { WorkspaceMode } from "@repo/base";

import { ModalWrapperProps } from "../types";

interface NewWorkspaceModalProps extends ModalWrapperProps {
  forceNewWindow?: boolean;
  window?: "welcome" | "main";
}

export const NewWorkspaceModal = ({
  closeModal,
  showModal,
  forceNewWindow = false,
  window = "main",
}: NewWorkspaceModalProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const { mutateAsync: createWorkspace, isPending: isCreatingWorkspace } = useCreateWorkspace();
  const { mutateAsync: openWorkspace, isPending: isOpeningWorkspace } = useOpenWorkspace();

  const { mutateAsync: openWelcomeWorkspace, isPending: isOpeningWelcomeWorkspace } = useWelcomeOpenWorkspace();
  const { mutateAsync: createWelcomeWorkspace, isPending: isCreatingWelcomeWorkspace } = useWelcomeCreateWorkspace();

  useFocusInputOnMount({ inputRef });

  const [name, setName] = useState("New Workspace");
  const [mode, setMode] = useState<WorkspaceMode>("LIVE");
  const [openInNewWindow, setOpenInNewWindow] = useState(true);

  // Force openInNewWindow to true when forceNewWindow prop is enabled
  const effectiveOpenInNewWindow = forceNewWindow ? true : openInNewWindow;

  const isLoadingWorkspaces =
    isCreatingWorkspace || isOpeningWorkspace || isCreatingWelcomeWorkspace || isOpeningWelcomeWorkspace;

  const handleSubmit = async () => {
    if (!name) return;

    if (window === "welcome") {
      const createWorkspaceOutput = await createWelcomeWorkspace({ name: name.trim() });
      await openWelcomeWorkspace({ id: createWorkspaceOutput.id });
    } else {
      await createWorkspace({
        name: name.trim(),
        openOnCreation: effectiveOpenInNewWindow ? OpenInTargetEnum.NEW_WINDOW : OpenInTargetEnum.CURRENT_WINDOW,
      });
    }

    closeModal();
    resetForm();
  };

  const handleCancel = () => {
    closeModal();
    resetForm();
  };

  const resetForm = () => {
    setTimeout(() => {
      setName("");
      setMode("LIVE");
      setOpenInNewWindow(true);
    }, 200);
  };

  return (
    <ModalForm
      title="New Workspace"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background) max-w-136"
      titleClassName="border-b border-(--moss-border)"
      footerClassName="border-t border-(--moss-border)"
      content={
        <div className="flex flex-col gap-2">
          <div className="gap-x-3.75 grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,1fr)] items-center py-4">
            <div className="self-center">Name:</div>
            <Input
              intent="outlined"
              ref={inputRef}
              value={name}
              className="max-w-72"
              onChange={(e) => setName(e.target.value)}
              pattern={VALID_NAME_PATTERN}
              required
            />
            <p className="text-(--moss-secondary-foreground) col-start-2 max-w-72 text-xs">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
          </div>

          <div>
            <div className="flex gap-2">
              <span>Mode</span>
              <div className="background-(--moss-border) my-auto h-px w-full" />
            </div>
            <p className="text-(--moss-secondary-foreground) text-xs leading-5">
              You can switch modes in the workspace at any time and as often as needed.
            </p>
            <div className="pl-5">
              <RadioGroup.Root>
                <RadioGroup.ItemWithLabel
                  label="Live mode"
                  description="Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for
                    planning and generating documentation upfront."
                  value="LIVE"
                  checked={mode === "LIVE"}
                  onClick={() => setMode("LIVE")}
                />

                <RadioGroup.ItemWithLabel
                  label="Design mode"
                  description="Begin by writing and testing requests, then define the API structure based on actual usage. Great
                    for quick prototyping and iterating."
                  value="DESIGN"
                  checked={mode === "DESIGN"}
                  onClick={() => setMode("DESIGN")}
                />
              </RadioGroup.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="py-0.75 flex items-center justify-between">
          {!forceNewWindow && (
            <CheckboxWithLabel
              label="Open in a new window?"
              checked={openInNewWindow}
              onCheckedChange={(check) => {
                if (check !== "indeterminate") setOpenInNewWindow(check);
              }}
            />
          )}
          {forceNewWindow && <div />}
          <div className="px-0.25 py-1.25 flex gap-3">
            <Button intent="outlined" type="button" onClick={handleCancel} disabled={isLoadingWorkspaces}>
              Close
            </Button>
            <Button intent="primary" disabled={name.length === 0 || isLoadingWorkspaces} type="submit">
              {isLoadingWorkspaces ? "Creating..." : "Create"}
            </Button>
          </div>
        </div>
      }
    />
  );
};
