import {
    createContext,
    useContext,
    useState,
    useRef,
    useEffect,
    Children,
    cloneElement,
    HTMLAttributes,
    forwardRef,
    ReactNode,
    isValidElement,
    ReactElement,
} from 'react';
import { cn } from '../utils';

const TabsContext = createContext<{
    activeIndex: number,
    setActiveIndex: (index: number) => void,
    tabListRef: React.RefObject<HTMLDivElement>,
    indicatorRef: React.RefObject<HTMLDivElement>,
    tabsRefs: React.MutableRefObject<(HTMLButtonElement | null)[]>,
}>({
    activeIndex: 0,
    setActiveIndex: () => { },
    tabListRef: { current: null },
    indicatorRef: { current: null },
    tabsRefs: { current: [] as (HTMLButtonElement | null)[] },
});

export const Tabs = ({ children, defaultIndex = 0, ...props }: {
    children: ReactNode;
    defaultIndex?: number;
} & HTMLAttributes<HTMLDivElement>) => {
    const [activeIndex, setActiveIndex] = useState(defaultIndex);
    const tabListRef = useRef<HTMLDivElement>(null);
    const indicatorRef = useRef<HTMLDivElement>(null);
    const tabsRefs = useRef<(HTMLButtonElement | null)[]>([]);

    useEffect(() => {
        const currentTab = tabsRefs.current[activeIndex];
        if (currentTab && indicatorRef.current) {
            const tabRect = currentTab.getBoundingClientRect();

            indicatorRef.current.style.width = `${tabRect.width}px`;
            indicatorRef.current.style.left = `${currentTab.offsetLeft}px`;
        }
    }, [activeIndex]);

    return (
        <TabsContext.Provider
            value={{ activeIndex, setActiveIndex, tabListRef, indicatorRef, tabsRefs }}
        >
            <div className="TabsRoot w-full flex flex-col" {...props}>
                {children}
            </div>
        </TabsContext.Provider>
    );
};

const TabsList = ({ children, ...props }: {
    children: ReactElement<React.ComponentProps<typeof Tab>> | ReactElement<React.ComponentProps<typeof Tab>>[]
}) => {
    const { tabListRef, indicatorRef, setActiveIndex, tabsRefs, activeIndex } =
        useContext(TabsContext);

    const mappedChildren = Children.map(children, (child, index) => {
        if (isValidElement(child)) {
            return cloneElement(child, {
                isActive: index === activeIndex,
                onClick: () => setActiveIndex(index),
                ref: (el: HTMLButtonElement | null) => (tabsRefs.current[index] = el),
            });
        }
        return child;
    });

    return (
        <div
            role="tablist"
            aria-labelledby="tablist-1"
            data-tabs="default"
            className={cn(`flex overflow-auto relative bg-[#F4F4F4] dark:bg-[#161819]`)}
            ref={tabListRef}
            {...props}
        >
            <Indicator ref={indicatorRef} />
            {mappedChildren}
        </div>
    );
};

const Indicator = forwardRef<HTMLDivElement>(({ }, ref) => {
    return <div
        data-tabs-indicator
        aria-hidden="true"
        className={`absolute transition-[left,width] bottom-auto top-0 h-[1px] bg-sky-600`}
        ref={ref}
    />
})

const Tab = forwardRef<
    HTMLButtonElement,
    { id: number | string, isActive?: boolean } & Omit<HTMLAttributes<HTMLButtonElement>, 'id'>
>(({ id, isActive = false, ...props }, ref) => (
    <button
        id={`${id}`}
        type="button"
        role="tab"
        aria-selected={isActive}
        aria-controls={`panel-${id}`}
        tabIndex={isActive ? 0 : -1}
        className={cn("min-w-max px-3 py-2.5 bg-[#F4F4F4] dark:bg-[#161819] dark:text-[#525252] cursor-pointer ", {
            "bg-white dark:bg-[#1e2021] dark:text-white": isActive,
            "hover:bg-white/50 hover:dark:bg-[#1e2021]/50 ": !isActive
        }, props.className)}
        ref={ref}
        {...props}
    >
        <span className="focus">{props.children}</span>
    </button>
));

const TabsPanels = ({ children, className, ...props }: {
    children: ReactNode;

} & HTMLAttributes<HTMLDivElement>) => {
    const { activeIndex } = useContext(TabsContext);
    const mappedPanels = Children.toArray(children).filter(child => isValidElement(child)).map((child, index) =>
        cloneElement(child as React.ReactElement, { isActive: index === activeIndex })
    );
    return <div className={cn("TabsPanels  w-full grow bg-white dark:bg-[#1e2021] overflow-auto", className)} {...props} >{mappedPanels}</div>;
};

const TabPanel = ({ children, id, isActive = false, ...props }: {
    children: ReactNode;
    id: string | number;
    isActive?: boolean;
} & Omit<HTMLAttributes<HTMLDivElement>, 'id'>) => (
    <div
        id={`panel-${id}`}
        role="tabpanel"
        tabIndex={0}
        aria-labelledby={`${id}`}
        className={cn("", {
            'hidden': !isActive
        })}
        {...props}
    >
        <p className="text-caption">{children}</p>
    </div>
);

Tabs.List = TabsList;
Tabs.Tab = Tab;
Tabs.Panels = TabsPanels;
Tabs.Panel = TabPanel;

export default Tabs;
