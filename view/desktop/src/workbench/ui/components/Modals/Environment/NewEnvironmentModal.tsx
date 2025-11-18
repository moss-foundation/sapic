import { useMemo, useRef, useState } from "react";

import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useFocusInputOnMount, useValidateInput } from "@/hooks";
import { Button } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import Input from "@/lib/ui/Input";
import { useCreateEnvironment, useStreamEnvironments } from "@/workbench/adapters";
import { useStreamProjects } from "@/workbench/adapters/tanstackQuery/project";
import { RadioGroup } from "@/workbench/ui/components";
import { useGroupedEnvironments } from "@/workbench/ui/components/EnvironmentsLists/hooks/useGroupedEnvironments";
import { ModalForm } from "@/workbench/ui/components/ModalForm";

import { ModalWrapperProps } from "../types";

export const NewEnvironmentModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const { globalEnvironments, projectEnvironments } = useStreamEnvironments();
  const { mutateAsync: createEnvironment } = useCreateEnvironment();
  const { data: projects } = useStreamProjects();
  const { groupedEnvironments } = useGroupedEnvironments();

  const [name, setName] = useState("New Environment");
  const [projectId, setProjectId] = useState<string | null>(null);
  const [mode, setMode] = useState<"Workspace" | "Project">("Workspace");
  const [openAutomatically, setOpenAutomatically] = useState(true);

  useFocusInputOnMount({
    inputRef,
    initialValue: name,
  });

  const restrictedNames = useMemo(() => {
    const list = mode === "Workspace" ? globalEnvironments : projectEnvironments;
    return list?.map((env) => env.name) ?? [];
  }, [mode, globalEnvironments, projectEnvironments]);

  const { isValid } = useValidateInput({
    value: name,
    restrictedValues: restrictedNames,
    inputRef,
  });

  const getNextOrder = (list?: { length?: number } | null) => (list?.length ?? 0) + 1;
  const handleSubmit = async () => {
    if (!isValid) return;

    if (mode === "Workspace") {
      await createEnvironment({
        name,
        order: getNextOrder(globalEnvironments),
        variables: [],
      });
    } else if (mode === "Project" && projectId) {
      const projectEnvironments = groupedEnvironments.find((group) => group.projectId === projectId)?.environments;

      await createEnvironment({
        name,
        order: getNextOrder(projectEnvironments),
        variables: [],
        projectId,
      });
    }

    closeModal();
  };

  const handleCancel = () => {
    closeModal();
  };

  const handleSelectProject = (value: string) => {
    setProjectId(value);
    setMode("Project");
  };

  return (
    <ModalForm
      title="New Environment"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background) max-w-136"
      titleClassName="border-b border-(--moss-border)"
      footerClassName="border-t border-(--moss-border)"
      content={
        <div className="flex flex-col gap-2">
          <div className="gap-x-3.75 grid grid-cols-[min-content_1fr] items-center gap-y-5 py-5">
            <div className="col-span-2 grid grid-cols-subgrid items-center gap-y-3">
              <div>Name:</div>
              <Input
                ref={inputRef}
                value={name}
                className="max-w-72"
                onChange={(e) => setName(e.target.value)}
                pattern={VALID_NAME_PATTERN}
                required
                intent="outlined"
              />
              <p className="text-(--moss-secondary-foreground) col-start-2 max-w-72 text-xs">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
            </div>
          </div>

          <div>
            <div className="flex gap-2">
              <span>Scope</span>
              <div className="background-(--moss-border) my-auto h-px w-full" />
            </div>
            <p className="text-(--moss-secondary-foreground) text-xs leading-5">
              You can switch modes in the workspace at any time and as often as needed.
            </p>
            <div className="pl-5">
              <RadioGroup.Root required>
                <RadioGroup.ItemWithLabel
                  label="Workspace"
                  description="This mode is suitable when your project is stored in a separate repository or doesn’t have a repository at all."
                  value="Workspace"
                  checked={mode === "Workspace"}
                  onClick={() => setMode("Workspace")}
                />

                <RadioGroup.ItemWithSelect
                  placeholder="Choose project"
                  label="Project"
                  description="This mode is suitable if you want to store the project in your project’s repository or in any other folder you specify."
                  value="Project"
                  checked={mode === "Project"}
                  onClick={() => setMode("Project")}
                  disabled={!projects || projects.length === 0}
                  options={projects?.map((project) => ({
                    label: project.name,
                    value: project.id,
                  }))}
                  selectValue={projectId ?? undefined}
                  onChange={handleSelectProject}
                  required={mode === "Project"}
                />
              </RadioGroup.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="py-0.75 flex items-center justify-between">
          <CheckboxWithLabel
            label="Activate after creation"
            checked={openAutomatically}
            onCheckedChange={(check) => {
              if (check !== "indeterminate") setOpenAutomatically(check);
            }}
          />
          <div className="px-0.25 py-1.25 flex gap-3">
            <Button intent="outlined" type="button" onClick={handleCancel}>
              Close
            </Button>
            <Button intent="primary" type="submit">
              Create
            </Button>
          </div>
        </div>
      }
    />
  );
};
