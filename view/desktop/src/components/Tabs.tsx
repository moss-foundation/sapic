import {
    useState,
    useRef,
    useEffect,
    HTMLAttributes,
    ReactNode,
    ReactElement,
} from 'react';
import { cn } from '../utils';
import {
    attachClosestEdge,
    extractClosestEdge,
    type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";
import DropIndicator from './DropIndicator';
import { createPortal } from 'react-dom';

interface TabsProps extends HTMLAttributes<HTMLDivElement> {
    children: ReactNode;
}

const Tabs = ({ children, className, ...props }: TabsProps) => {
    return (
        <div className={cn("w-full flex flex-col", className)} {...props}>
            {children}
        </div>
    );
};

interface TabsListProps extends HTMLAttributes<HTMLDivElement> {
    children: ReactElement<React.ComponentProps<typeof Tab>> | ReactElement<React.ComponentProps<typeof Tab>>[];
}

const TabsList = ({ children, className, ...props }: TabsListProps) => {
    return (
        <div
            role="tablist"
            aria-labelledby="tablist-1"
            data-tabs="default"
            className={cn(`flex overflow-auto relative bg-[#F4F4F4] dark:bg-[#161819]`, className)}
            {...props}
        >
            {children}
        </div>
    );
};

interface TabProps extends Omit<HTMLAttributes<HTMLButtonElement>, 'id'> {
    id: number | string,
    isActive: boolean,
    isDraggable?: boolean
    label: string
    draggableType?: string
}

const Tab = ({ id, isActive, isDraggable = false, className, draggableType = "TabTrigger", label, ...props }: TabProps) => {
    const ref = useRef<HTMLButtonElement | null>(null);
    const [preview, setPreview] = useState<HTMLElement | null>(null);
    const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

    useEffect(() => {
        const element = ref?.current;

        if (!element || !isDraggable) return;

        return combine(
            draggable({
                element,
                getInitialData: () => ({ id, label }),
                onDrop: () => {
                    setPreview(null);
                },
                onGenerateDragPreview({ nativeSetDragImage }) {
                    setCustomNativeDragPreview({
                        nativeSetDragImage,
                        render({ container }) {
                            setPreview((prev) => (prev === container ? prev : container));
                        },
                    });
                },
            }),
            dropTargetForElements({
                element,
                onDrop: () => {
                    setClosestEdge(null);
                },
                getData({ input }) {
                    return attachClosestEdge(
                        { id, label, draggableType },
                        {
                            element,
                            input,
                            allowedEdges: ["right", "left"],
                        }
                    );
                },
                getIsSticky() {
                    return true;
                },
                onDragEnter({ self }) {
                    const closestEdge = extractClosestEdge(self.data);
                    setClosestEdge(closestEdge);
                },
                onDrag({ self }) {
                    const closestEdge = extractClosestEdge(self.data);
                    setClosestEdge((current) => {
                        if (current === closestEdge) return current;

                        return closestEdge;
                    });
                },
                onDragLeave() {
                    setClosestEdge(null);
                },
            })
        );
    }, [id, label, isDraggable, draggableType, ref]);

    return <button
        ref={ref}
        id={`${id}`}
        type="button"
        role="tab"
        aria-selected={isActive}
        aria-controls={`panel-${id}`}
        tabIndex={isActive ? 0 : -1}
        className={cn("relative min-w-max px-3 pb-2 pt-[7px] bg-[#F4F4F4] dark:bg-[#161819] dark:text-[#525252] cursor-pointer border-t box-border", {
            "bg-white dark:bg-[#1e2021] dark:text-white border-[#0065FF] ": isActive,
            "hover:bg-white/50 hover:dark:bg-[#1e2021]/50 border-transparent": !isActive
        }, className)}
        {...props}
    >
        <span className="focus">{label}</span>
        {closestEdge ? <DropIndicator edge={closestEdge} gap={0} noTerminal /> : null}
        {preview && createPortal(<Tab id={id} label={label} isActive={isActive} />, preview)}
    </button>
};

interface TabsPanelsProps extends HTMLAttributes<HTMLDivElement> {
    children: ReactNode;
}

const TabsPanels = ({ children, className, ...props }: TabsPanelsProps) => {
    return <div className={cn("w-full grow bg-white dark:bg-[#1e2021] overflow-auto", className)} {...props} >{children}</div>;
};

interface TabPanelProps extends Omit<HTMLAttributes<HTMLDivElement>, 'id'> {
    children: ReactNode;
    id: string | number;
    isActive: boolean;
}

const TabPanel = ({ children, id, isActive, className, ...props }: TabPanelProps) => {
    return <div
        id={`panel-${id}`}
        role="tabpanel"
        tabIndex={0}
        aria-labelledby={`${id}`}
        className={cn("", {
            'hidden': !isActive
        }, className)}
        {...props}
    >
        <p className="text-caption">{children}</p>
    </div>
}

Tabs.List = TabsList;
Tabs.Tab = Tab;
Tabs.Panels = TabsPanels;
Tabs.Panel = TabPanel;

export default Tabs;
