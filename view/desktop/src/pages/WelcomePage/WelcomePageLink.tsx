import { Icon } from "@/components";

export interface WelcomePageLinkProps extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  label: string;
  withIcon?: boolean;
}

export const WelcomePageLink = ({ label, withIcon, ...props }: WelcomePageLinkProps) => {
  return (
    <a className="inline-flex cursor-pointer items-center text-(--moss-primary)" {...props}>
      <span className="hover:underline">{label}</span> {withIcon && <Icon icon="ExternalLink" />}
    </a>
  );
};

export default WelcomePageLink;
