import { OverlayScrollbarsComponent, OverlayScrollbarsComponentProps } from "overlayscrollbars-react";

interface ScrollbarProps extends OverlayScrollbarsComponentProps {
  children: React.ReactNode;
}

const defaultProps: OverlayScrollbarsComponentProps = {
  options: {
    scrollbars: {
      autoHide: "move",
    },
  },
  defer: true,
};

export const Scrollbar = ({ children, ...props }: ScrollbarProps) => {
  return (
    <OverlayScrollbarsComponent {...defaultProps} {...props}>
      {children}
    </OverlayScrollbarsComponent>
  );
};

export default Scrollbar;
