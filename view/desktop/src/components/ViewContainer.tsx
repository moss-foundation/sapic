import { useState } from "react";

import { useCreateWorkspace } from "@/hooks/useCreateWorkspace";
import { useGetViewGroup } from "@/hooks/useGetViewGroup";
import { useGetWorkspaces } from "@/hooks/useGetWorkspaces";
import { useModal } from "@/hooks/useModal";
import { useOpenWorkspace } from "@/hooks/useOpenWorkspace";

import Button from "./Button";
import CollectionTreeView from "./CollectionTreeView";
import { Checkbox, Icon, Input, Radio } from "./index";
import { Modal } from "./Modal";
import Select from "./Select";

export const ViewContainer = ({ groupId }: { groupId: string }) => {
  const { data: workspaces, isLoading } = useGetWorkspaces();
  const { data: viewGroup } = useGetViewGroup(groupId);

  // if (!workspaces || workspaces.length === 0 || isLoading) {
  return (
    <div className="flex h-full flex-col">
      <NoWorkspaceComponent />
    </div>
  );
  // }

  if (!viewGroup) {
    return <div>No view group found</div>;
  }

  switch (groupId) {
    case "collections.groupId":
      return <CollectionTreeView />;
    case "environments.groupId":
      return <div>No view group found</div>;
    case "mock.groupId":
      return <div>No view group found</div>;
    default:
      return <div>No view group found</div>;
  }
};

const NoWorkspaceComponent = () => {
  const {
    showModal: showNewWorkspaceModal,
    closeModal: closeNewWorkspaceModal,
    openModal: openNewWorkspaceModal,
  } = useModal();
  const {
    showModal: showOpenWorkspaceModal,
    closeModal: closeOpenWorkspaceModal,
    openModal: openOpenWorkspaceModal,
  } = useModal();

  return (
    <div className="flex flex-col gap-4.25 px-2">
      <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      <div>
        <Icon icon="ErrorNaughtyDog" className="mx-auto size-[200px] w-full" />
        <p className="text-(--moss-secondary-text)">
          You need to open a workspace before accessing collections, environments, or sending requests. Please open or
          create a workspace to proceed.
        </p>
      </div>

      <div className="flex flex-col gap-3.5">
        {/* //TODO This should be a button component */}
        <button
          onClick={openNewWorkspaceModal}
          className="background-(--moss-primary) flex cursor-pointer items-center justify-center rounded py-1.5 text-white"
        >
          New workspace
        </button>
        {/* //TODO This should be a button component */}
        <button
          onClick={openOpenWorkspaceModal}
          className="background-(--moss-primary) flex cursor-pointer items-center justify-center rounded py-1.5 text-white"
        >
          Open workspace
        </button>
      </div>
    </div>
  );
};

const NewWorkspaceModal = ({ closeModal, showModal }: { showModal: boolean; closeModal: () => void }) => {
  const { mutate: createWorkspace } = useCreateWorkspace();

  const [name, setName] = useState("");
  const [radioList, setRadioList] = useState([
    {
      id: "RequestFirstMode",
      label: "Request-first mode",
      description:
        "Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for planning and generating documentation upfront.",
      checked: true,
    },
    {
      id: "DesignFirstMode",
      label: "Design-first mode",
      description:
        "Begin by writing and testing requests, then define the API structure based on actual usage. Great for quick prototyping and iterating.",
      checked: false,
    },
  ]);

  const handleSubmit = async () => {
    if (name) {
      createWorkspace({ name });
      closeModal();
    }
  };

  return (
    <Modal
      title="New Workspace"
      onBackdropClick={closeModal}
      showModal={showModal}
      onSubmit={handleSubmit}
      content={
        <div className="flex flex-col gap-2">
          <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,1fr)] items-center gap-3">
            <div>Name:</div>
            <Input
              value={name}
              variant="outlined"
              className="max-w-72"
              required
              onChange={(e) => setName(e.target.value)}
            />
            <p className="col-start-2 max-w-72 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
          </div>

          <div>
            <div className="flex gap-2">
              <span>Mode</span>
              <div className="background-(--moss-border-color) my-auto h-px w-full" />
            </div>
            <div className="pl-5">
              <Radio.Root>
                {radioList.map((radio) => (
                  <div
                    key={radio.id}
                    className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,min-content)] items-center gap-x-2"
                  >
                    <Radio.Item
                      value={radio.id}
                      id={radio.id}
                      checked={radio.checked}
                      onClick={() =>
                        setRadioList((list) =>
                          list.map((item) =>
                            item.id === radio.id ? { ...item, checked: true } : { ...item, checked: false }
                          )
                        )
                      }
                    >
                      <Radio.Indicator>
                        <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
                      </Radio.Indicator>
                    </Radio.Item>

                    <label htmlFor={radio.id} className="cursor-pointer py-2">
                      {radio.label}
                    </label>
                    <p className="col-start-2 text-left text-(--moss-secondary-text)">{radio.description}</p>
                  </div>
                ))}
              </Radio.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-between">
          <div className="flex gap-2">
            <Checkbox.Root id="c1" className="cursor-pointer">
              <Checkbox.Indicator>
                <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
              </Checkbox.Indicator>
            </Checkbox.Root>
            <label htmlFor="c1" className="cursor-pointer">
              Open automatically after creation
            </label>
          </div>
          <div className="flex gap-2">
            <Button variant="outlined" intent="neutral" onClick={closeModal}>
              Close
            </Button>
            {/* //TODO This should be a button component */}
            <button
              type="submit"
              className="background-(--moss-primary) hover:background-(--moss-blue-3) flex cursor-pointer items-center justify-center rounded px-3.75 py-1.5 text-white"
            >
              Create
            </button>
          </div>
        </div>
      }
    />
  );
};

const OpenWorkspaceModal = ({ closeModal, showModal }: { showModal: boolean; closeModal: () => void }) => {
  const { data: workspaces } = useGetWorkspaces();

  const [radioList, setRadioList] = useState([
    {
      id: "RequestFirstMode",
      label: "Request-first mode",
      description:
        "Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for planning and generating documentation upfront.",
      checked: true,
    },
    {
      id: "DesignFirstMode",
      label: "Design-first mode",
      description:
        "Begin by writing and testing requests, then define the API structure based on actual usage. Great for quick prototyping and iterating.",
      checked: false,
    },
  ]);

  const [selectedWorkspace, setSelectedWorkspace] = useState<string | undefined>(undefined);

  const { mutate: openWorkspace } = useOpenWorkspace();

  const handleSubmit = () => {
    if (selectedWorkspace) {
      openWorkspace(selectedWorkspace);
      closeModal();
    }
  };

  return (
    <Modal
      title="Open Workspace"
      onBackdropClick={closeModal}
      showModal={showModal}
      onSubmit={handleSubmit}
      content={
        <div className="flex flex-col gap-2">
          <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(1,1fr)] items-center gap-3">
            <div>Name:</div>

            <Select.Root onValueChange={setSelectedWorkspace} value={selectedWorkspace}>
              <Select.Trigger className="flex w-56 justify-between">
                <Select.Value placeholder="Select workspace" />
                <Icon icon="ChevronDown" />
              </Select.Trigger>

              <Select.Content className="z-50" position="popper">
                <Select.Viewport>
                  {workspaces?.map((workspace) => (
                    <Select.Item value={workspace.name} key={workspace.name}>
                      <Select.ItemText>{workspace.name}</Select.ItemText>
                    </Select.Item>
                  ))}
                </Select.Viewport>
              </Select.Content>
            </Select.Root>
          </div>

          <div>
            <div className="flex gap-2">
              <span>Mode</span>
              <div className="background-(--moss-border-color) my-auto h-px w-full" />
            </div>
            <div className="pl-5">
              <Radio.Root>
                {radioList.map((radio) => (
                  <div
                    key={radio.id}
                    className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,min-content)] items-center gap-x-2"
                  >
                    <Radio.Item
                      value={radio.id}
                      id={radio.id}
                      checked={radio.checked}
                      onClick={() =>
                        setRadioList((list) =>
                          list.map((item) => {
                            return {
                              ...item,
                              checked: item.id === radio.id,
                            };
                          })
                        )
                      }
                    >
                      <Radio.Indicator>
                        <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
                      </Radio.Indicator>
                    </Radio.Item>

                    <label
                      htmlFor={radio.id}
                      className="cursor-pointer py-2"
                      onClick={() =>
                        setRadioList((list) =>
                          list.map((item) => {
                            return {
                              ...item,
                              checked: item.id === radio.id,
                            };
                          })
                        )
                      }
                    >
                      {radio.label}
                    </label>
                    <span className="col-start-2 text-left text-(--moss-secondary-text)">{radio.description}</span>
                  </div>
                ))}
              </Radio.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-between">
          <div className="flex gap-2">
            <Checkbox.Root id="OpenAutomaticallyAfterCreationId" className="cursor-pointer">
              <Checkbox.Indicator>
                <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
              </Checkbox.Indicator>
            </Checkbox.Root>
            <label htmlFor="OpenAutomaticallyAfterCreationId" className="cursor-pointer">
              Open automatically after creation
            </label>
          </div>
          <div className="flex gap-2">
            <Button variant="outlined" intent="neutral" onClick={closeModal}>
              Close
            </Button>
            {/* //TODO This should be a button component */}
            <button
              type="submit"
              className="background-(--moss-primary) hover:background-(--moss-blue-3) flex cursor-pointer items-center justify-center rounded px-3.75 py-1.5 text-white"
            >
              Open
            </button>
          </div>
        </div>
      }
    />
  );
};
