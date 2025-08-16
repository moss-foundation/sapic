import { ButtonNeutralOutlined, ButtonPrimary } from "@/components";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const ButtonsSection = () => {
  return (
    <KitchenSinkSection
      header="Button Components"
      description="Various button states and variants available in the application."
    >
      <div className="flex gap-2">
        <ButtonPrimary>ButtonPrimary</ButtonPrimary>
        <ButtonPrimary disabled>ButtonPrimaryDisabled</ButtonPrimary>
      </div>
      <div className="flex gap-2">
        <ButtonNeutralOutlined>ButtonNeutralOutlined</ButtonNeutralOutlined>
        <ButtonNeutralOutlined disabled>ButtonNeutralOutlinedDisabled</ButtonNeutralOutlined>
      </div>
    </KitchenSinkSection>
  );
};
