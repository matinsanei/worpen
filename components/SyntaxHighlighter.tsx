import React, { useMemo } from 'react';

interface SyntaxHighlighterProps {
    code: string;
    language?: 'json' | 'yaml';
    className?: string;
}

interface Token {
    type: 'keyword' | 'string' | 'number' | 'boolean' | 'null' | 'operator' | 'punctuation' | 'property' | 'comment' | 'text';
    value: string;
}

// Dynamic Routes Engine Keywords
const ROUTE_KEYWORDS = [
    // Control Flow
    'if', 'then', 'otherwise', 'else', 'switch', 'cases', 'default', 'while', 'loop', 'for_each', 'until',
    'break', 'continue', 'return',
    
    // Error Handling
    'try', 'catch', 'finally', 'throw',
    
    // Operations
    'set', 'get', 'query_db', 'http_request', 'execute_script', 'parallel', 'define_function', 'call_function',
    'map', 'filter', 'aggregate', 'reduce', 'transform',
    
    // Route Definition
    'name', 'description', 'path', 'method', 'logic', 'parameters', 'enabled', 'version', 'auth_required',
    
    // Variables & Context
    'var', 'value', 'condition', 'body', 'operations', 'collection', 'params', 'args',
    
    // HTTP & Database
    'url', 'query', 'language', 'code', 'timeout', 'retry', 'headers',
    
    // Data Operations
    'operation', 'input', 'output', 'tasks', 'max_concurrent', 'max_iterations',
    
    // Helper Functions (example subset)
    'upper', 'lower', 'trim', 'split', 'join', 'replace', 'hash', 'base64',
    'uuid', 'now', 'format_date', 'parse_json', 'to_json',
    
    // Aggregations
    'sum', 'count', 'avg', 'min', 'max',
];

const HTTP_METHODS = ['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD', 'OPTIONS'];

/**
 * Tokenize YAML code into syntax-aware tokens
 */
const tokenizeYAML = (code: string): Token[] => {
    const tokens: Token[] = [];
    const lines = code.split('\n');

    for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
        const line = lines[lineIndex];
        let i = 0;

        // Add indentation
        while (i < line.length && line[i] === ' ') {
            tokens.push({ type: 'text', value: line[i] });
            i++;
        }

        // Comments
        if (line[i] === '#') {
            tokens.push({ type: 'comment', value: line.slice(i) });
            tokens.push({ type: 'text', value: '\n' });
            continue;
        }

        // Parse rest of line
        while (i < line.length) {
            const char = line[i];

            // List indicator
            if (char === '-' && (i === 0 || line[i - 1] === ' ') && (i + 1 >= line.length || line[i + 1] === ' ')) {
                tokens.push({ type: 'punctuation', value: '-' });
                i++;
                if (i < line.length && line[i] === ' ') {
                    tokens.push({ type: 'text', value: ' ' });
                    i++;
                }
                continue;
            }

            // Property key (before colon)
            if (char !== ' ' && char !== ':') {
                let key = '';
                while (i < line.length && line[i] !== ':' && line[i] !== ' ') {
                    key += line[i];
                    i++;
                }

                // Check if next non-space is colon
                let j = i;
                while (j < line.length && line[j] === ' ') j++;
                
                if (j < line.length && line[j] === ':') {
                    // It's a property key
                    if (ROUTE_KEYWORDS.includes(key.toLowerCase())) {
                        tokens.push({ type: 'keyword', value: key });
                    } else {
                        tokens.push({ type: 'property', value: key });
                    }
                    
                    // Add spaces before colon
                    while (i < j) {
                        tokens.push({ type: 'text', value: line[i] });
                        i++;
                    }
                    continue;
                }

                // Not a property, treat as value
                if (key === 'true' || key === 'false') {
                    tokens.push({ type: 'boolean', value: key });
                } else if (key === 'null') {
                    tokens.push({ type: 'null', value: key });
                } else if (/^-?\d+(\.\d+)?$/.test(key)) {
                    tokens.push({ type: 'number', value: key });
                } else if (ROUTE_KEYWORDS.includes(key.toLowerCase()) || HTTP_METHODS.includes(key)) {
                    tokens.push({ type: 'keyword', value: key });
                } else {
                    tokens.push({ type: 'string', value: key });
                }
                continue;
            }

            // Colon
            if (char === ':') {
                tokens.push({ type: 'punctuation', value: ':' });
                i++;
                if (i < line.length && line[i] === ' ') {
                    tokens.push({ type: 'text', value: ' ' });
                    i++;
                }
                continue;
            }

            // String value (after colon)
            if (char !== ' ') {
                let value = '';
                let isQuoted = false;
                
                // Quoted string
                if (char === '"' || char === "'") {
                    const quote = char;
                    isQuoted = true;
                    value += char;
                    i++;
                    while (i < line.length && line[i] !== quote) {
                        if (line[i] === '\\') {
                            value += line[i] + (line[i + 1] || '');
                            i += 2;
                        } else {
                            value += line[i];
                            i++;
                        }
                    }
                    if (i < line.length) {
                        value += line[i];
                        i++;
                    }
                } else {
                    // Unquoted value
                    while (i < line.length && line[i] !== '#') {
                        value += line[i];
                        i++;
                    }
                    value = value.trimEnd();
                }

                // Determine value type
                const trimmedValue = value.replace(/^["']|["']$/g, '');
                if (value === 'true' || value === 'false') {
                    tokens.push({ type: 'boolean', value });
                } else if (value === 'null') {
                    tokens.push({ type: 'null', value });
                } else if (!isQuoted && /^-?\d+(\.\d+)?$/.test(value)) {
                    tokens.push({ type: 'number', value });
                } else if (ROUTE_KEYWORDS.includes(trimmedValue.toLowerCase()) || HTTP_METHODS.includes(trimmedValue)) {
                    tokens.push({ type: 'keyword', value });
                } else {
                    tokens.push({ type: 'string', value });
                }
                continue;
            }

            // Space
            tokens.push({ type: 'text', value: char });
            i++;
        }

        // Newline
        if (lineIndex < lines.length - 1) {
            tokens.push({ type: 'text', value: '\n' });
        }
    }

    return tokens;
};

/**
 * Tokenize JSON code into syntax-aware tokens
 */
const tokenizeJSON = (code: string): Token[] => {
    const tokens: Token[] = [];
    let i = 0;

    while (i < code.length) {
        const char = code[i];

        // Skip whitespace
        if (/\s/.test(char)) {
            tokens.push({ type: 'text', value: char });
            i++;
            continue;
        }

        // Comments (not standard JSON but useful for YAML-style)
        if (char === '/' && code[i + 1] === '/') {
            let comment = '';
            while (i < code.length && code[i] !== '\n') {
                comment += code[i];
                i++;
            }
            tokens.push({ type: 'comment', value: comment });
            continue;
        }

        // Strings
        if (char === '"') {
            let str = '"';
            i++;
            let isProperty = false;
            
            while (i < code.length && code[i] !== '"') {
                if (code[i] === '\\') {
                    str += code[i] + (code[i + 1] || '');
                    i += 2;
                } else {
                    str += code[i];
                    i++;
                }
            }
            str += '"';
            i++;

            // Check if this string is followed by a colon (property key)
            let j = i;
            while (j < code.length && /\s/.test(code[j])) j++;
            if (code[j] === ':') isProperty = true;

            const strContent = str.slice(1, -1);
            
            // Check if it's a keyword
            if (ROUTE_KEYWORDS.includes(strContent.toLowerCase())) {
                tokens.push({ type: 'keyword', value: str });
            } else if (HTTP_METHODS.includes(strContent)) {
                tokens.push({ type: 'keyword', value: str });
            } else if (isProperty) {
                tokens.push({ type: 'property', value: str });
            } else {
                tokens.push({ type: 'string', value: str });
            }
            continue;
        }

        // Numbers
        if (/[0-9]/.test(char) || (char === '-' && /[0-9]/.test(code[i + 1]))) {
            let num = '';
            while (i < code.length && /[0-9.\-eE+]/.test(code[i])) {
                num += code[i];
                i++;
            }
            tokens.push({ type: 'number', value: num });
            continue;
        }

        // Booleans
        if (code.substr(i, 4) === 'true') {
            tokens.push({ type: 'boolean', value: 'true' });
            i += 4;
            continue;
        }
        if (code.substr(i, 5) === 'false') {
            tokens.push({ type: 'boolean', value: 'false' });
            i += 5;
            continue;
        }

        // Null
        if (code.substr(i, 4) === 'null') {
            tokens.push({ type: 'null', value: 'null' });
            i += 4;
            continue;
        }

        // Operators and Punctuation
        if ('{}[]:,'.includes(char)) {
            tokens.push({ type: 'punctuation', value: char });
            i++;
            continue;
        }

        // Default: text
        tokens.push({ type: 'text', value: char });
        i++;
    }

    return tokens;
};

/**
 * Get color class for token type
 */
const getTokenColor = (type: Token['type']): string => {
    const colors: Record<Token['type'], string> = {
        keyword: 'text-purple-400 font-semibold',
        string: 'text-green-400',
        number: 'text-blue-400',
        boolean: 'text-orange-400',
        null: 'text-gray-500 italic',
        operator: 'text-cyan-400',
        punctuation: 'text-gray-400',
        property: 'text-yellow-400',
        comment: 'text-gray-600 italic',
        text: 'text-gray-300',
    };
    return colors[type] || 'text-gray-300';
};

/**
 * Syntax Highlighter Component
 */
export const SyntaxHighlighter: React.FC<SyntaxHighlighterProps> = ({ 
    code, 
    language = 'json',
    className = '' 
}) => {
    const tokens = useMemo(() => {
        if (language === 'yaml') {
            return tokenizeYAML(code);
        }
        return tokenizeJSON(code);
    }, [code, language]);

    return (
        <pre className={`font-mono text-xs leading-[18px] ${className}`}>
            <code>
                {tokens.map((token, i) => (
                    <span key={i} className={getTokenColor(token.type)}>
                        {token.value}
                    </span>
                ))}
            </code>
        </pre>
    );
};

/**
 * Enhanced Textarea with Syntax Highlighting Overlay
 */
interface CodeEditorProps {
    value: string;
    onChange: (value: string) => void;
    placeholder?: string;
    className?: string;
    language?: 'json' | 'yaml';
}

export const CodeEditor: React.FC<CodeEditorProps> = ({
    value,
    onChange,
    placeholder,
    className = '',
    language = 'json'
}) => {
    const lines = value.split('\n');
    
    // Auto-detect language if not specified
    const detectedLanguage = language === 'json' && !value.trim().startsWith('{') && !value.trim().startsWith('[')
        ? 'yaml'
        : language;

    return (
        <div className={`relative group ${className} overflow-hidden`}>
            {/* Scrollable Container */}
            <div className="absolute inset-0 overflow-auto custom-scrollbar">
                <div className="relative min-h-full">
                    {/* Line Numbers */}
                    <div className="absolute left-0 top-0 w-12 bg-black/80 border-r border-white/10 text-[10px] font-mono text-gray-600 select-none z-10">
                        <div className="pt-4 pb-4">
                            {lines.map((_, i) => (
                                <div key={i} className="h-[18px] px-2 text-right leading-[18px]">
                                    {i + 1}
                                </div>
                            ))}
                        </div>
                    </div>

                    {/* Syntax Highlighting Overlay */}
                    <div className="absolute left-12 top-0 right-0 pt-4 pb-4 px-4 pointer-events-none z-0">
                        <SyntaxHighlighter code={value} language={detectedLanguage} />
                    </div>

                    {/* Actual Editable Textarea */}
                    <textarea
                        value={value}
                        onChange={(e) => onChange(e.target.value)}
                        placeholder={placeholder}
                        className="w-full min-h-full bg-transparent border-0 rounded pl-16 pr-4 pt-4 pb-4 text-xs font-mono text-transparent caret-green-400 focus:outline-none resize-none leading-[18px] relative z-20"
                        style={{ 
                            tabSize: 2,
                            fontFamily: "'JetBrains Mono', 'Fira Code', 'Consolas', monospace",
                            caretColor: '#4ade80', // green-400
                            height: `${Math.max(lines.length * 18 + 32, 400)}px` // Dynamic height based on content
                        }}
                        spellCheck={false}
                    />
                </div>
            </div>
        </div>
    );
};
