import { IDETheme } from './types';

export const hybrex: IDETheme = {
    id: 'hybrex',
    name: 'Hybrex',
    activityBar: {
        bg: '#0a0e14',
        border: '#1a1f29',
        activeIcon: '#00d9ff',
        activeBg: '#00d9ff15',
        inactiveIcon: '#4d5766',
        hoverBg: '#151922',
        indicator: '#00d9ff',
    },
    sidebar: {
        bg: '#0d1117',
        border: '#1a1f29',
        headerText: '#8b949e',
        folderText: '#f0f6fc',
        folderHoverBg: '#161b22',
        itemText: '#c9d1d9',
        itemHoverBg: '#161b22',
        activeItemBg: '#00d9ff20',
        activeItemText: '#00d9ff',
        iconEnabled: '#3fb950',
        iconDisabled: '#6e7681',
    },
    editor: {
        bg: '#010409',
        border: '#1a1f29',
        tabBg: '#0d1117',
        tabActiveBg: '#010409',
        tabActiveText: '#f0f6fc',
        tabInactiveText: '#7d8590',
        tabBorder: '#1a1f29',
        tabIndicator: '#00d9ff',
        monacoTheme: 'hybrex',
        monacoConfig: {
            base: 'vs-dark',
            inherit: true,
            rules: [
                // Keywords - Cyan bright
                { token: 'keyword', foreground: '00d9ff', fontStyle: 'bold' },
                { token: 'keyword.control', foreground: '00d9ff', fontStyle: 'bold' },
                
                // Strings - Green emerald
                { token: 'string', foreground: '3fb950' },
                { token: 'string.quoted', foreground: '3fb950' },
                
                // Numbers - Purple bright
                { token: 'number', foreground: 'bc8cff' },
                { token: 'constant.numeric', foreground: 'bc8cff' },
                
                // Comments - Gray muted
                { token: 'comment', foreground: '6e7681', fontStyle: 'italic' },
                { token: 'comment.line', foreground: '6e7681', fontStyle: 'italic' },
                { token: 'comment.block', foreground: '6e7681', fontStyle: 'italic' },
                
                // Types & Classes - Yellow gold
                { token: 'type', foreground: 'ffa657', fontStyle: 'bold' },
                { token: 'entity.name.type', foreground: 'ffa657', fontStyle: 'bold' },
                { token: 'entity.name.class', foreground: 'ffa657', fontStyle: 'bold' },
                { token: 'support.type', foreground: 'ffa657' },
                { token: 'support.class', foreground: 'ffa657' },
                
                // Functions - Orange bright
                { token: 'function', foreground: 'ff7b72' },
                { token: 'entity.name.function', foreground: 'ff7b72' },
                { token: 'support.function', foreground: 'ff7b72' },
                { token: 'meta.function-call', foreground: 'ff7b72' },
                
                // Variables - Light blue
                { token: 'variable', foreground: '79c0ff' },
                { token: 'variable.parameter', foreground: '79c0ff' },
                { token: 'variable.other', foreground: '79c0ff' },
                
                // Operators - Cyan
                { token: 'operator', foreground: '00d9ff' },
                { token: 'keyword.operator', foreground: '00d9ff' },
                
                // Identifiers - White bright
                { token: 'identifier', foreground: 'f0f6fc' },
                
                // Properties - Pink
                { token: 'property', foreground: 'ffa198' },
                { token: 'variable.property', foreground: 'ffa198' },
                
                // Tags - Green
                { token: 'tag', foreground: '7ee787' },
                { token: 'entity.name.tag', foreground: '7ee787' },
                
                // Attributes - Light cyan
                { token: 'attribute', foreground: '79c0ff' },
                { token: 'entity.other.attribute-name', foreground: '79c0ff' },
                
                // Constants - Purple
                { token: 'constant', foreground: 'bc8cff' },
                { token: 'constant.language', foreground: 'bc8cff', fontStyle: 'bold' },
                
                // Storage - Cyan bright
                { token: 'storage', foreground: '00d9ff', fontStyle: 'bold' },
                { token: 'storage.type', foreground: '00d9ff', fontStyle: 'bold' },
                
                // Punctuation - Gray light
                { token: 'punctuation', foreground: 'c9d1d9' },
                
                // Meta - Gray
                { token: 'meta', foreground: '8b949e' },
                
                // Invalid - Red
                { token: 'invalid', foreground: 'ff6e6e', fontStyle: 'underline' },
            ],
            colors: {
                'editor.background': '#010409',
                'editor.foreground': '#f0f6fc',
                'editor.lineHighlightBackground': '#161b22',
                'editor.selectionBackground': '#00d9ff30',
                'editor.selectionHighlightBackground': '#00d9ff20',
                'editor.wordHighlightBackground': '#00d9ff15',
                'editor.findMatchBackground': '#ffa65780',
                'editor.findMatchHighlightBackground': '#ffa65740',
                'editorCursor.foreground': '#00d9ff',
                'editorLineNumber.foreground': '#6e7681',
                'editorLineNumber.activeForeground': '#c9d1d9',
                'editorIndentGuide.background': '#21262d',
                'editorIndentGuide.activeBackground': '#30363d',
                'editorWhitespace.foreground': '#484f58',
                'editorRuler.foreground': '#21262d',
                'editorBracketMatch.background': '#00d9ff20',
                'editorBracketMatch.border': '#00d9ff',
                'editorError.foreground': '#ff6e6e',
                'editorWarning.foreground': '#ffa657',
                'editorInfo.foreground': '#79c0ff',
                'editorHint.foreground': '#8b949e',
                'editorWidget.background': '#161b22',
                'editorWidget.border': '#30363d',
                'editorSuggestWidget.background': '#161b22',
                'editorSuggestWidget.border': '#30363d',
                'editorSuggestWidget.selectedBackground': '#238636',
                'editorHoverWidget.background': '#161b22',
                'editorHoverWidget.border': '#30363d',
                'scrollbar.shadow': '#010409',
                'scrollbarSlider.background': '#21262d80',
                'scrollbarSlider.hoverBackground': '#30363d80',
                'scrollbarSlider.activeBackground': '#6e768180',
            },
        },
    },
    statusBar: {
        bg: '#0d1117',
        text: '#7d8590',
        hoverBg: '#161b22',
        border: '#1a1f29',
    },
};
