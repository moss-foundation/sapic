import { Icon } from "@/lib/ui";

export const WelcomeViewSteps = () => {
  return (
    <div className="flex w-full flex-col gap-2">
      <h3 className="text-xl">Next steps</h3>
      <div className="flex min-w-[600px] flex-wrap gap-3">
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
    <div className="background-(--moss-secondary-background) flex max-w-[275px] flex-col gap-1.5 rounded-lg px-4 py-3">
      <div className="flex items-center gap-1.5">
        <Icon icon="Info" />
        <span className="font-medium">Learn the Fundamentals</span>
        {isNew && (
          <div className="background-(--moss-stepCard-background) text-(--moss-stepCard-foreground) rounded-[3px] px-1 text-[11px] font-medium">
            New
          </div>
        )}
      </div>

      <div className="text-(--moss-secondary-foreground)">
        Explain behavior that is not clear from the setting or action name.
      </div>
    </div>
  );
};

export default WelcomeViewSteps;
