import { Icon } from "@/components";
import { NewWorkspaceModal } from "@/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/components/Modals/Workspace/OpenWorkspaceModal";
import { useModal } from "@/hooks/useModal";

import WelcomePageDivider from "./WelcomePageDivider";
import WelcomePageLink from "./WelcomePageLink";
import WelcomePageRecentWorkspaces from "./WelcomePageRecentWorkspaces";
import WelcomePageSteps from "./WelcomePageSteps";

export const WelcomePage = () => {
  return (
    <div className="relative min-h-screen select-none">
      <div className="relative flex h-full min-w-min flex-col gap-6 px-[20px] pt-32 lg:px-[60px] xl:px-[140px]">
        <div className="flex flex-col gap-0.5">
          <h1 className="text-[34px]">Simple API Client</h1>
          <p className="text-lg text-(--moss-secondary-text)">Design APIs, Send Requests, Unmatched Git Integration</p>
        </div>

        <div className="flex flex-col gap-7.5">
          <div className="grid grid-cols-2">
            <FirstColumn />
            <SecondColumn />
          </div>

          <WelcomePageSteps />
        </div>

        <div className="mt-auto mb-8 flex justify-center">
          <div className="flex flex-col items-center gap-2 text-sm">
            <span>Learn more</span>
            <Icon icon="ChevronDownEllipse" />
          </div>
        </div>
      </div>

      <div className="flex h-screen w-full flex-col items-center pb-6">
        <div id="TestAnchorForWelcomePage" className="mt-auto">
          hello
        </div>
      </div>
    </div>
  );
};

const FirstColumn = () => {
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
    <>
      <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />

      <div className="flex flex-col gap-7.5">
        <div className="flex flex-col items-start gap-2">
          <h2 className="text-lg">Start</h2>
          <button className="flex cursor-pointer gap-1.5" onClick={openNewWorkspaceModal}>
            <Icon icon="FolderAdd" className="size-4 text-(--moss-primary)" />
            <span>New workspace</span>
          </button>
          <button className="flex cursor-pointer gap-1.5" onClick={openOpenWorkspaceModal}>
            <Icon icon="Folder" className="size-4 text-(--moss-primary)" />
            <span>Open workspace</span>
          </button>
        </div>

        <WelcomePageRecentWorkspaces />
      </div>
    </>
  );
};

const SecondColumn = () => {
  return (
    <div className="flex max-w-[268px] flex-col gap-2 justify-self-end">
      <h2 className="text-xl">Pin board</h2>
      <div>
        <p className="text-(--moss-secondary-text)">Lorem ipsum dolor sitel, consectetur adipiscing.</p>

        <WelcomePageDivider />

        <p className="text-(--moss-secondary-text)">
          Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        </p>

        <WelcomePageDivider />

        <WelcomePageLink label="View Sapic’s Roadmap" withIcon />

        <WelcomePageDivider />

        <div className="flex flex-col items-start gap-2">
          <h3>Release pages:</h3>
          <div className="flex flex-col gap-2">
            <WelcomePageLink label="Quisque Faucibus" withIcon />
            <WelcomePageLink label="Tempus Leo" withIcon />
            <WelcomePageLink label="Lacinia Integer" withIcon />
          </div>
        </div>
      </div>
    </div>
  );
};

export default WelcomePage;
