import { NewWorkspaceModal } from "@/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/components/Modals/Workspace/OpenWorkspaceModal";
import { useModal } from "@/hooks/useModal";
import { Icon } from "@/lib/ui";

import WelcomePageDivider from "./WelcomePageDivider";
import WelcomePageLink from "./WelcomePageLink";
import WelcomePageRecentWorkspaces from "./WelcomePageRecentWorkspaces";
import WelcomePageSteps from "./WelcomePageSteps";

export const WelcomePage = () => {
  const handleLearnMoreClick = (e: React.MouseEvent) => {
    e.preventDefault();
    const element = document.getElementById("TestAnchorForWelcomePage");
    if (!element) {
      return;
    }
    element.scrollIntoView({ behavior: "smooth" });
  };
  return (
    <div className="relative z-[50] h-full overflow-auto">
      <section className="relative flex min-h-[calc(100vh-98px)] flex-col gap-6 px-[20px] pt-32 lg:px-[60px] xl:px-[140px]">
        <div className="flex flex-col gap-0.5">
          <h1 className="fill-[var(--moss-gray-6)] text-[34px]">Simple API Client</h1>

          <p className="text-lg text-(--moss-secondary-text)">Design APIs, Send Requests, Unmatched Git Integration</p>
        </div>

        <div className="flex flex-col gap-7.5">
          <div className="grid grid-cols-[minmax(0px,1fr)_1fr]">
            <FirstColumn />
            <SecondColumn />
          </div>

          <WelcomePageSteps />
        </div>

        <a
          href="#TestAnchorForWelcomePage"
          onClick={handleLearnMoreClick}
          className="group/learn-more relative bottom-8 mt-auto flex cursor-pointer flex-col items-center gap-2 self-center pt-10 text-sm"
        >
          <span>Learn more</span>
          <Icon
            icon="ChevronDownHovered"
            className="group-hover/learn-more:background-(--moss-icon-primary-background-hover) rounded-full transition-colors"
          />
        </a>
      </section>

      <section id="TestAnchorForWelcomePage" className="flex h-screen flex-col items-center justify-center bg-red-50">
        <p className="text-2xl font-bold">Lorem ipsum</p>
      </section>
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
      {showNewWorkspaceModal && (
        <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      )}
      {showOpenWorkspaceModal && (
        <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      )}

      <div className="flex flex-col gap-7.5">
        <div className="flex flex-col items-start gap-2">
          <h2 className="text-lg">Start</h2>
          <button className="flex cursor-pointer gap-1.5" onClick={openNewWorkspaceModal}>
            <Icon icon="NewWorkspaceActive" className="size-4 text-(--moss-primary)" />
            <span>New workspace</span>
          </button>
          <button className="flex cursor-pointer gap-1.5" onClick={openOpenWorkspaceModal}>
            <Icon icon="OpenWorkspaceActive" className="size-4 text-(--moss-primary)" />
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

        <WelcomePageLink label="View Sapic's Roadmap" withIcon />

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
