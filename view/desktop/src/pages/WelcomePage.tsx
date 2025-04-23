import { Icon } from "@/components";
import Button from "@/components/Button";
import { NewWorkspaceModal } from "@/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/components/Modals/Workspace/OpenWorkspaceModal";
import { useModal } from "@/hooks/useModal";

export const WelcomePage = () => {
  return (
    <div className="@container h-full">
      <div className="relative flex h-full min-w-min flex-col gap-7.5 pt-32 pr-12 pl-12 @xl:pr-[140px] @xl:pl-[140px]">
        <div className="flex flex-col gap-4">
          <h1 className="text-[34px] font-medium">Simple API Client</h1>
          <p className="text-lg font-medium text-(--moss-secondary-text)">
            Design APIs, Send Requests, Unmatched Git Integration
          </p>
        </div>

        <div className="grid grid-cols-2">
          <FirstColumn />
          <SecondColumn />
        </div>

        <StepsRow />

        <ScrollToAnchor />
      </div>

      <div className="flex h-screen w-full flex-col items-center pb-6">
        <div id="TestAnchorForWelcomePage" className="mt-auto">
          hello
        </div>
      </div>
    </div>
  );
};

const StepsRow = () => {
  return (
    <div className="flex flex-col gap-2">
      <h3 className="text-xl font-medium">Next steps</h3>
      <div className="flex flex-col lg:flex-row lg:gap-4 lg:pl-4">
        <StepCard isNew />
        <StepCard />
        <StepCard />
        <StepCard isNew />
      </div>
    </div>
  );
};

const StepCard = ({ isNew = false }: { isNew?: boolean }) => {
  return (
    <div className="background-(--moss-secondary-background) max-w-[275px] min-w-[225px] rounded-lg">
      <div className="flex items-center gap-1.5">
        <Icon icon="StepCardInfo" />
        <span className="font-medium">Learn the Fundamentals</span>
        {isNew && (
          <div className="background-(--moss-stepCard-bg) rounded-[3px] px-1 text-[11px] font-semibold text-(--moss-stepCard-text)">
            New
          </div>
        )}
      </div>
      <div className="text-(--moss-secondary-text)">
        Explain behavior that is not clear from the setting or action name.
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
        <div className="flex flex-col gap-2">
          <h2 className="text-lg font-medium">Start</h2>
          <button className="flex cursor-pointer gap-1.5" onClick={openNewWorkspaceModal}>
            <Icon icon="FolderAdd" className="size-4 text-(--moss-primary)" />
            <span>New workspace</span>
          </button>
          <button className="flex cursor-pointer gap-1.5" onClick={openOpenWorkspaceModal}>
            <Icon icon="Folder" className="size-4 text-(--moss-primary)" />
            <span>Open workspace</span>
          </button>
        </div>

        <div className="flex flex-col gap-2">
          <h2 className="text-lg font-medium">Recent</h2>
          <div className="flex flex-col gap-1.5">
            <WelcomePageLink label="My Workspace" />
            <WelcomePageLink label="Spaixel Monster" />
            <WelcomePageLink label="Twinkle" />
          </div>
          <div>
            <Button variant="outlined" intent="neutral">
              More
            </Button>
          </div>
        </div>
      </div>
    </>
  );
};

const SecondColumn = () => {
  return (
    <div className="flex max-w-[268px] flex-col gap-2 justify-self-end">
      <h2 className="text-xl font-medium">Pin board</h2>
      <div>
        <p className="text-(--moss-secondary-text)">Lorem ipsum dolor sitel, consectetur adipiscing.</p>

        <WelcomePageDivider />

        <p className="text-(--moss-secondary-text)">
          Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        </p>

        <WelcomePageDivider />

        <WelcomePageLink label="View Sapic’s Roadmap" withIcon />

        <WelcomePageDivider />

        <div className="flex flex-col gap-2">
          <h3 className="font-medium">Release pages:</h3>
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

const WelcomePageLink = ({ label, withIcon }: { label: string; withIcon?: boolean }) => {
  return (
    <a href="#" className="flex items-center text-(--moss-primary)">
      <span className="hover:underline">{label}</span> {withIcon && <Icon icon="ExternalLink" />}
    </a>
  );
};

const WelcomePageDivider = () => {
  return <div className="background-(--moss-border-color) my-3 h-px w-full" />;
};

const ScrollToAnchor = () => {
  return (
    <div className="mt-auto mb-8 flex justify-center">
      <div className="flex flex-col items-center gap-2 text-sm">
        <span className="font-medium">Learn more</span>
        <Icon icon="ChevronDownEllipse" />
      </div>
    </div>
  );
};

export default WelcomePage;
