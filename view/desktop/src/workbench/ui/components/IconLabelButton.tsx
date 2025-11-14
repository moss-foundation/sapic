import { ButtonHTMLAttributes, forwardRef, KeyboardEvent, useEffect, useRef, useState } from "react";

import { Icon, type Icons } from "@/lib/ui";
import { cn } from "@/utils";

interface IconLabelButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  leftIcon?: Icons;
  rightIcon?: Icons;
  title: string;
  placeholder?: string;
  className?: string;
  leftIconClassName?: string;
  rightIconClassName?: string;
  labelClassName?: string;
  placeholderClassName?: string;
  compact?: boolean;
  showPlaceholder?: boolean;
  editable?: boolean;
  onRename?: (newName: string) => void;
}

interface LabelProps {
  title: string;
  placeholder?: string;
  className?: string;
  placeholderClassName?: string;
  showPlaceholder?: boolean;
  editable?: boolean;
  isEditing: boolean;
  onStartEdit: () => void;
  onRename: (newName: string) => void;
}

const buttonStyles = `
  group
  flex items-center
  h-[22px] min-w-0  
  cursor-pointer rounded p-[3px]
  text-(--moss-controls-foreground) 
  hover:background-(--moss-controls-background-hover) 
  disabled:cursor-default 
  disabled:opacity-50
  truncate
`;

const ButtonLabel = ({
  title,
  placeholder,
  className,
  placeholderClassName,
  showPlaceholder,
  editable,
  isEditing,
  onStartEdit,
  onRename,
}: LabelProps) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const spanRef = useRef<HTMLSpanElement>(null);
  const [inputValue, setInputValue] = useState(title);

  useEffect(() => {
    if (isEditing && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.select();
    }
  }, [isEditing]);

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      onRename(inputValue);
    } else if (e.key === "Escape") {
      setInputValue(title);
      onRename(title);
    }
  };

  const handleBlur = () => {
    onRename(inputValue);
  };

  if (isEditing && editable) {
    return (
      <input
        ref={inputRef}
        type="text"
        value={inputValue}
        onChange={(e) => setInputValue(e.target.value)}
        onKeyDown={handleKeyDown}
        onBlur={handleBlur}
        className="text-md background-(--moss-controls-background) text-(--moss-controls-foreground) h-[18px] min-w-[80px] px-1 outline-none"
      />
    );
  }

  if (showPlaceholder && placeholder) {
    return (
      <span
        ref={spanRef}
        className={cn("text-md truncate", placeholderClassName)}
        onDoubleClick={editable ? onStartEdit : undefined}
      >
        {placeholder}
      </span>
    );
  }

  return (
    <span
      ref={spanRef}
      className={cn("text-md text-(--moss-controls-foreground) truncate text-left", className)}
      onDoubleClick={editable ? onStartEdit : undefined}
    >
      {title}
    </span>
  );
};

export const IconLabelButton = forwardRef<HTMLButtonElement, IconLabelButtonProps>(
  (
    {
      leftIcon,
      rightIcon,
      title,
      placeholder,
      className,
      leftIconClassName,
      rightIconClassName,
      labelClassName,
      placeholderClassName,
      compact = false,
      showPlaceholder = false,
      editable = false,
      onRename,
      ...props
    },
    ref
  ) => {
    const [isEditing, setIsEditing] = useState(false);
    const labelContainerRef = useRef<HTMLDivElement>(null);

    const handleStartEdit = () => {
      if (editable) {
        setIsEditing(true);
      }
    };

    const handleRename = (newName: string) => {
      setIsEditing(false);
      if (onRename && newName.trim() !== "") {
        onRename(newName);
      }
    };

    useEffect(() => {
      // Handle the programmatic double-click from the parent component
      if (editable && ref && typeof ref !== "function" && ref.current) {
        const button = ref.current;

        const handleDoubleClick = (e: MouseEvent) => {
          // Ensures we only process the event when it's a real double-click or our programmatic one
          if (editable && !isEditing) {
            handleStartEdit();
            e.stopPropagation();
          }
        };

        button.addEventListener("dblclick", handleDoubleClick);

        return () => {
          button.removeEventListener("dblclick", handleDoubleClick);
        };
      }
      return () => {};
    }, [editable, ref, isEditing, handleStartEdit]);

    return (
      <button
        ref={ref}
        className={cn(buttonStyles, className)}
        onClick={(e) => {
          if (!isEditing && props.onClick) {
            props.onClick(e);
          }
        }}
        {...props}
      >
        <div
          ref={labelContainerRef}
          className={compact ? "flex items-center gap-0.5" : "flex items-center gap-1 truncate px-1"}
        >
          {leftIcon && <Icon icon={leftIcon} className={cn("size-4", leftIconClassName)} />}
          {!compact && (
            <ButtonLabel
              title={title}
              placeholder={placeholder}
              className={cn("mx-0.5", labelClassName)}
              placeholderClassName={cn("mx-0.5", placeholderClassName)}
              showPlaceholder={showPlaceholder}
              editable={editable}
              isEditing={isEditing}
              onStartEdit={handleStartEdit}
              onRename={handleRename}
            />
          )}
          {rightIcon && <Icon icon={rightIcon} className={cn("size-4", rightIconClassName)} />}
        </div>
      </button>
    );
  }
);

export default IconLabelButton;
