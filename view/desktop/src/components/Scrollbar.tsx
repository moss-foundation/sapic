import { OverlayScrollbarsComponent, OverlayScrollbarsComponentProps } from "overlayscrollbars-react";

interface ScrollbarProps extends OverlayScrollbarsComponentProps {
  children: React.ReactNode;
  className?: string;
}

const defaultProps: OverlayScrollbarsComponentProps = {
  options: {
    scrollbars: {
      autoHide: "move",
    },
  },
  defer: true,
};

export const Scrollbar = ({ children, className, ...props }: ScrollbarProps) => {
  return (
    <OverlayScrollbarsComponent className={className} {...defaultProps} {...props}>
      {children}
    </OverlayScrollbarsComponent>
  );
};

export default Scrollbar;
