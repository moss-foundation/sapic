import { Icon } from "@/lib/ui";

export interface WelcomePageLinkProps extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  label: string;
  withIcon?: boolean;
}

export const WelcomePageLink = ({ label, withIcon, ...props }: WelcomePageLinkProps) => {
  return (
    <a className="flex w-full min-w-0 cursor-pointer items-center text-(--moss-primary)" {...props}>
      <span className="truncate hover:underline">{label}</span>
      {withIcon && <Icon icon="ExternalLinkActive" />}
    </a>
  );
};

export default WelcomePageLink;
