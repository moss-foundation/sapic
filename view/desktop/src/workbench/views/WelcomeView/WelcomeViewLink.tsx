import { IconInline } from "@/workbench/ui/components/IconInline";

export interface WelcomePageLinkProps extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  label: string;
  withIcon?: boolean;
}

export const WelcomeViewLink = ({ label, withIcon, ...props }: WelcomePageLinkProps) => {
  return (
    <a className="text-(--moss-primary) flex w-full min-w-0 cursor-pointer items-center" {...props}>
      <span className="truncate hover:underline">{label}</span>
      {withIcon && <IconInline icon="ExternalLinkActive" />}
    </a>
  );
};

export default WelcomeViewLink;
