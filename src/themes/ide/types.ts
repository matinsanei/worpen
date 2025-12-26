
export interface IDETheme {
    id: string;
    name: string;
    activityBar: {
        bg: string;
        border: string;
        activeIcon: string;
        activeBg: string;
        inactiveIcon: string;
        hoverBg: string;
        indicator: string;
    };
    sidebar: {
        bg: string;
        border: string;
        headerText: string;
        folderText: string;
        folderHoverBg: string;
        itemText: string;
        itemHoverBg: string;
        activeItemBg: string;
        activeItemText: string;
        iconEnabled: string;
        iconDisabled: string;
    };
    editor: {
        bg: string;
        border: string;
        tabBg: string;
        tabActiveBg: string;
        tabActiveText: string;
        tabInactiveText: string;
        tabBorder: string;
        tabIndicator: string;
        monacoTheme: string;
        monacoConfig: {
            base: 'vs' | 'vs-dark' | 'hc-black';
            inherit: boolean;
            rules: Array<{
                token: string;
                foreground?: string;
                background?: string;
                fontStyle?: string;
            }>;
            colors: {
                [key: string]: string;
            };
        };
    };
    statusBar: {
        bg: string;
        text: string;
        hoverBg: string;
        border: string;
    };
}
