import { useRef } from "react";

import { useListWorkspaces } from "@/adapters/tanstackQuery/workspace";
import { useModal } from "@/hooks/useModal";
import { Icon, Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";
import { NewWorkspaceModal } from "@/workbench/ui/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/workbench/ui/components/Modals/Workspace/OpenWorkspaceModal";
import { DefaultViewProps } from "@/workbench/ui/parts/TabbedPane/types";

import WelcomeViewDivider from "./WelcomeViewDivider";
import WelcomeViewLink from "./WelcomeViewLink";
import WelcomeViewRecentWorkspaces from "./WelcomeViewRecentWorkspaces";
import WelcomeViewSteps from "./WelcomeViewSteps";

export type WelcomeViewProps = DefaultViewProps;

export const WelcomeView = ({}: WelcomeViewProps) => {
  const learnMoreRef = useRef<HTMLAnchorElement>(null);

  const handleLearnMoreClick = (e: React.MouseEvent) => {
    e.preventDefault();
    learnMoreRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  return (
    <Scrollbar className="relative h-full">
      <section className="relative flex min-h-[calc(100vh-98px)] flex-col gap-6 px-[20px] pt-32 lg:px-[60px] xl:px-[140px]">
        <div className="flex flex-col gap-4 leading-6">
          <h1 className="text-[34px]">Simple API Client</h1>

          <p className="text-(--moss-secondary-foreground) text-pretty text-lg">
            Design APIs, Send Requests, Unmatched Git Integration
          </p>
        </div>

        <div className="gap-7.5 flex flex-col">
          <div className="grid grid-cols-[minmax(0px,1fr)_1fr]">
            <FirstColumn />
            <SecondColumn />
          </div>

          <WelcomeViewSteps />
        </div>

        <a
          ref={learnMoreRef}
          href="#TestAnchorForWelcomePage"
          onClick={handleLearnMoreClick}
          className="group/learn-more relative bottom-8 mt-auto flex cursor-pointer flex-col items-center gap-2 self-center pt-10 text-sm"
        >
          <span>Learn more</span>
          <Icon
            icon="ChevronDownHovered"
            className="group-hover/learn-more:background-(--moss-primary-background-hover) rounded-full transition-colors"
          />
        </a>
      </section>

      <section id="TestAnchorForWelcomePage" className="flex h-screen flex-col items-center justify-center bg-red-50">
        <p className="text-2xl font-bold">Lorem ipsum</p>
      </section>
    </Scrollbar>
  );
};

const FirstColumn = () => {
  const { data: workspaces } = useListWorkspaces();

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
      <div className="gap-7.5 flex flex-col">
        <div className="flex flex-col items-start gap-2">
          <h2 className="text-lg">Start</h2>
          <button className="flex cursor-pointer gap-1.5" onClick={openNewWorkspaceModal}>
            <Icon icon="NewWorkspace" className="text-(--moss-accent) size-4" />
            <span>New workspace</span>
          </button>

          <button
            disabled={workspaces?.length === 0}
            className={cn("flex gap-1.5", {
              "cursor-not-allowed opacity-50": workspaces?.length === 0,
              "cursor-pointer": workspaces?.length && workspaces?.length > 0,
            })}
            onClick={workspaces?.length && workspaces?.length > 0 ? openOpenWorkspaceModal : undefined}
          >
            <Icon icon="Workspace" className="text-(--moss-accent) size-4" />
            <span>Open workspace</span>
          </button>
        </div>

        <WelcomeViewRecentWorkspaces />
      </div>

      {showNewWorkspaceModal && (
        <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      )}
      {showOpenWorkspaceModal && (
        <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      )}
    </>
  );
};

const SecondColumn = () => {
  return (
    <div className="flex max-w-[268px] flex-col gap-2 justify-self-end">
      <h2 className="text-xl">Pin board</h2>
      <div>
        <p className="text-(--moss-secondary-foreground)">Lorem ipsum dolor sitel, consectetur adipiscing.</p>

        <WelcomeViewDivider />

        <p className="text-(--moss-secondary-foreground)">
          Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        </p>

        <WelcomeViewDivider />

        <WelcomeViewLink label="View Sapic's Roadmap" withIcon />

        <WelcomeViewDivider />

        <div className="flex flex-col items-start gap-2">
          <h3>Release pages:</h3>
          <div className="flex flex-col gap-2">
            <WelcomeViewLink label="Quisque Faucibus" withIcon />
            <WelcomeViewLink label="Tempus Leo" withIcon />
            <WelcomeViewLink label="Lacinia Integer" withIcon />
          </div>
        </div>
      </div>
    </div>
  );
};

export default WelcomeView;
