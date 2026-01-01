
import React, { useState, useEffect, useCallback } from 'react';
import { functionsApi, FunctionDef, FunctionRegistryResponse } from '../api';
import Editor from "@monaco-editor/react";
import { 
  RefreshCw, Code2, AlertCircle, Loader2, Plus, Search, 
  Save, Trash2, X, FileCode, AlertTriangle, CheckCircle, FunctionSquare, Sparkles, MessageSquare
} from 'lucide-react';
import { AIGeneratorModal } from '../components/AIGeneratorModal';
import { CopilotSidebar } from '../components/CopilotSidebar';

export const FunctionsView: React.FC = () => {
  const [functions, setFunctions] = useState<Record<string, FunctionDef>>({});
  const [count, setCount] = useState<number>(0);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');
  
  // Editor state
  const [activeFunction, setActiveFunction] = useState<FunctionDef | null>(null);
  const [isCreatingNew, setIsCreatingNew] = useState<boolean>(false);
  const [editedName, setEditedName] = useState<string>('');
  const [editedParams, setEditedParams] = useState<string>('');
  const [editedLogic, setEditedLogic] = useState<string>('[]');
  const [saveError, setSaveError] = useState<string | null>(null);
  const [saveSuccess, setSaveSuccess] = useState<string | null>(null);
  const [saving, setSaving] = useState<boolean>(false);
  const [deleting, setDeleting] = useState<boolean>(false);
  const [showAIModal, setShowAIModal] = useState<boolean>(false);
  const [showCopilot, setShowCopilot] = useState<boolean>(false);

  const loadFunctions = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data: FunctionRegistryResponse = await functionsApi.list();
      setFunctions(data.functions);
      setCount(data.count);
    } catch (err: any) {
      setError(err.message || 'Failed to fetch global functions');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadFunctions();
  }, [loadFunctions]);

  const handleSelectFunction = (func: FunctionDef) => {
    setActiveFunction(func);
    setIsCreatingNew(false);
    setEditedName(func.name);
    setEditedParams(func.params.join(', '));
    setEditedLogic(JSON.stringify(func.logic, null, 2));
    setSaveError(null);
    setSaveSuccess(null);
  };

  const handleNewFunction = () => {
    setIsCreatingNew(true);
    setActiveFunction(null);
    setEditedName('');
    setEditedParams('');
    setEditedLogic('[\n  {\n    "return": {\n      "value": "TODO"\n    }\n  }\n]');
    setSaveError(null);
    setSaveSuccess(null);
  };

  const handleSave = async () => {
    setSaveError(null);
    setSaveSuccess(null);

    // Validation
    if (!editedName.trim()) {
      setSaveError('Function name is required');
      return;
    }

    let parsedLogic: any[];
    try {
      parsedLogic = JSON.parse(editedLogic);
      if (!Array.isArray(parsedLogic)) {
        throw new Error('Logic must be an array');
      }
    } catch (err: any) {
      setSaveError(`Invalid JSON: ${err.message}`);
      return;
    }

    const paramsArray = editedParams
      .split(',')
      .map(p => p.trim())
      .filter(p => p.length > 0);

    const payload: FunctionDef = {
      name: editedName.trim(),
      params: paramsArray,
      logic: parsedLogic,
    };

    try {
      setSaving(true);
      await functionsApi.create(payload);
      setSaveSuccess(`Function "${payload.name}" registered successfully!`);
      await loadFunctions();
      
      // Keep the editor open with the saved function
      setTimeout(() => {
        const savedFunc = Object.values(functions).find(f => f.name === payload.name);
        if (savedFunc) {
          handleSelectFunction(savedFunc);
        } else {
          // If not found immediately, it will be in the next load
          setIsCreatingNew(false);
        }
      }, 100);
    } catch (err: any) {
      setSaveError(err.message || 'Failed to register function');
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = async () => {
    if (!activeFunction) return;
    
    const confirmDelete = window.confirm(
      `Are you sure you want to delete function "${activeFunction.name}"?`
    );
    if (!confirmDelete) return;

    try {
      setDeleting(true);
      await functionsApi.delete(activeFunction.name);
      setSaveSuccess(`Function "${activeFunction.name}" deleted successfully!`);
      setActiveFunction(null);
      setIsCreatingNew(false);
      await loadFunctions();
    } catch (err: any) {
      setSaveError(err.message || 'Failed to delete function');
    } finally {
      setDeleting(false);
    }
  };

  const handleClose = () => {
    setActiveFunction(null);
    setIsCreatingNew(false);
    setSaveError(null);
    setSaveSuccess(null);
  };

  const handleAIGenerated = (code: string) => {
    setEditedLogic(code);
    setSaveSuccess('AI generated logic inserted! Review and save when ready.');
  };

  const handleCopilotApply = (code: string) => {
    // Try to parse as JSON array or insert as-is
    const trimmedCode = code.trim();
    
    try {
      // If it's valid JSON, format it nicely
      const parsed = JSON.parse(trimmedCode);
      setEditedLogic(JSON.stringify(parsed, null, 2));
    } catch {
      // If not valid JSON, just append to existing logic
      setEditedLogic(editedLogic + '\n' + trimmedCode);
    }
    
    setSaveSuccess('Code applied from Copilot! Review and save when ready.');
  };

  const filteredFunctions = Object.entries(functions).filter(([key, func]) =>
    func.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  if (loading) {
    return (
      <div className="h-full flex items-center justify-center bg-[#1e1f22]">
        <div className="flex flex-col items-center gap-3 text-[#6e7073]">
          <Loader2 size={32} className="animate-spin text-[#3574f0]" />
          <p className="text-sm">Loading function registry...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="h-full flex items-center justify-center bg-[#1e1f22]">
        <div className="flex flex-col items-center gap-3 text-[#e06c75]">
          <AlertCircle size={32} />
          <p className="text-sm">{error}</p>
          <button
            onClick={loadFunctions}
            className="px-4 py-2 bg-[#3574f0] text-white rounded-[var(--radius)] hover:bg-[#2d5fc7] transition-colors text-sm"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full flex bg-[#1e1f22] overflow-hidden">
      {/* LEFT SIDEBAR - Function List */}
      <div className="w-80 flex flex-col border-r border-[#43454a] bg-[#2b2d30]">
        {/* Sidebar Header */}
        <div className="p-3 border-b border-[#43454a] bg-[#2b2d30]">
          <div className="flex items-center gap-2 mb-3">
            <div className="w-8 h-8 rounded-[var(--radius)] bg-[#3574f0]/10 border border-[#3574f0]/30 flex items-center justify-center">
              <FunctionSquare size={16} className="text-[#3574f0]" />
            </div>
            <div className="flex-1">
              <h2 className="text-sm font-bold text-[#dfe1e5]">Functions Lib</h2>
              <p className="text-[10px] text-[#6e7073]">{count} registered</p>
            </div>
            <button
              onClick={loadFunctions}
              className="p-1.5 text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40] rounded-[var(--radius)] transition-colors"
              title="Refresh"
            >
              <RefreshCw size={14} />
            </button>
          </div>

          {/* New Function Button */}
          <button
            onClick={handleNewFunction}
            className="w-full flex items-center justify-center gap-2 px-3 py-2 bg-[#3574f0] hover:bg-[#2d5fc7] text-white text-sm font-medium rounded-[var(--radius)] transition-all active:scale-95"
          >
            <Plus size={16} />
            New Function
          </button>

          {/* Search Bar */}
          <div className="mt-3 relative">
            <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-[#6e7073]" />
            <input
              type="text"
              placeholder="Search functions..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-9 pr-3 py-2 bg-[#1e1f22] border border-[#43454a] rounded-[var(--radius)] text-[#dfe1e5] text-xs placeholder-[#6e7073] focus:outline-none focus:border-[#3574f0] transition-colors"
            />
          </div>
        </div>

        {/* Function List */}
        <div className="flex-1 overflow-y-auto custom-scrollbar">
          {filteredFunctions.length === 0 ? (
            <div className="p-8 text-center text-[#6e7073]">
              <FileCode size={32} className="mx-auto mb-2 opacity-30" />
              <p className="text-xs">
                {searchQuery ? 'No matching functions' : 'No functions yet'}
              </p>
            </div>
          ) : (
            <div className="p-2 space-y-1">
              {filteredFunctions.map(([key, func]) => (
                <button
                  key={key}
                  onClick={() => handleSelectFunction(func)}
                  className={`w-full text-left p-3 rounded-[var(--radius)] transition-all group ${
                    activeFunction?.name === func.name && !isCreatingNew
                      ? 'bg-[#3574f0]/10 border border-[#3574f0]/30'
                      : 'hover:bg-[#393b40] border border-transparent'
                  }`}
                >
                  <div className="flex items-center gap-2 mb-1">
                    <Code2 size={14} className="text-[#3574f0] flex-shrink-0" />
                    <span className="text-sm font-medium text-[#dfe1e5] font-mono truncate">
                      {func.name}
                    </span>
                  </div>
                  {func.params.length > 0 && (
                    <div className="flex flex-wrap gap-1 mt-2">
                      {func.params.slice(0, 3).map((param, idx) => (
                        <span
                          key={idx}
                          className="px-1.5 py-0.5 text-[10px] font-medium rounded bg-[#6e7073]/20 text-[#6e7073]"
                        >
                          {param}
                        </span>
                      ))}
                      {func.params.length > 3 && (
                        <span className="px-1.5 py-0.5 text-[10px] font-medium text-[#6e7073]">
                          +{func.params.length - 3}
                        </span>
                      )}
                    </div>
                  )}
                </button>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* RIGHT PANEL - Editor */}
      <div className="flex-1 flex flex-col">
        {!activeFunction && !isCreatingNew ? (
          // Empty State
          <div className="h-full flex items-center justify-center text-[#6e7073]">
            <div className="text-center max-w-md">
              <FunctionSquare size={64} className="mx-auto mb-4 opacity-20" />
              <h3 className="text-lg font-bold text-[#dfe1e5] mb-2">No Function Selected</h3>
              <p className="text-sm mb-6">
                Select a function from the list or create a new one to get started.
              </p>
              <button
                onClick={handleNewFunction}
                className="inline-flex items-center gap-2 px-6 py-3 bg-[#3574f0] hover:bg-[#2d5fc7] text-white font-medium rounded-[var(--radius)] transition-all active:scale-95"
              >
                <Plus size={18} />
                Create New Function
              </button>
            </div>
          </div>
        ) : (
          <>
            {/* Editor Header */}
            <div className="h-12 flex items-center justify-between px-4 border-b border-[#43454a] bg-[#2b2d30]">
              <div className="flex items-center gap-3">
                <Code2 size={18} className="text-[#3574f0]" />
                <h3 className="text-sm font-bold text-[#dfe1e5]">
                  {isCreatingNew ? 'New Function' : activeFunction?.name}
                </h3>
              </div>
              <div className="flex items-center gap-2">
                {!isCreatingNew && activeFunction && (
                  <button
                    onClick={handleDelete}
                    disabled={deleting}
                    className="flex items-center gap-1.5 px-3 py-1.5 bg-[#e06c75]/10 hover:bg-[#e06c75]/20 text-[#e06c75] text-xs font-medium rounded-[var(--radius)] transition-all disabled:opacity-50"
                  >
                    <Trash2 size={12} />
                    {deleting ? 'Deleting...' : 'Delete'}
                  </button>
                )}
                <button
                  onClick={() => setShowCopilot(!showCopilot)}
                  className="flex items-center gap-1.5 px-3 py-1.5 bg-[#c678dd]/20 hover:bg-[#c678dd]/30 border border-[#c678dd]/30 text-[#c678dd] text-xs font-bold rounded-[var(--radius)] transition-all active:scale-95"
                >
                  <MessageSquare size={12} />
                  Copilot
                </button>
                <button
                  onClick={() => setShowAIModal(true)}
                  className="flex items-center gap-1.5 px-4 py-1.5 bg-gradient-to-r from-[#3574f0] to-[#c678dd] hover:from-[#2d5fc7] hover:to-[#a85ec4] text-white text-xs font-bold rounded-[var(--radius)] transition-all active:scale-95 shadow-lg"
                >
                  <Sparkles size={12} />
                  Generate with AI
                </button>
                <button
                  onClick={handleSave}
                  disabled={saving}
                  className="flex items-center gap-1.5 px-4 py-1.5 bg-[#59a869] hover:bg-[#4a9158] text-white text-xs font-bold rounded-[var(--radius)] transition-all active:scale-95 disabled:opacity-50"
                >
                  <Save size={12} />
                  {saving ? 'Saving...' : 'Save'}
                </button>
                <button
                  onClick={handleClose}
                  className="p-1.5 text-[#6e7073] hover:text-[#dfe1e5] hover:bg-[#393b40] rounded-[var(--radius)] transition-colors"
                >
                  <X size={16} />
                </button>
              </div>
            </div>

            {/* Notifications */}
            {(saveError || saveSuccess) && (
              <div className={`mx-4 mt-3 p-3 rounded-[var(--radius)] border flex items-start gap-2 ${
                saveError 
                  ? 'bg-[#e06c75]/10 border-[#e06c75]/30 text-[#e06c75]' 
                  : 'bg-[#59a869]/10 border-[#59a869]/30 text-[#59a869]'
              }`}>
                {saveError ? <AlertTriangle size={16} className="flex-shrink-0 mt-0.5" /> : <CheckCircle size={16} className="flex-shrink-0 mt-0.5" />}
                <p className="text-xs flex-1">{saveError || saveSuccess}</p>
                <button 
                  onClick={() => { setSaveError(null); setSaveSuccess(null); }}
                  className="text-current hover:opacity-70 transition-opacity"
                >
                  <X size={14} />
                </button>
              </div>
            )}

            {/* Editor Content */}
            <div className="flex-1 overflow-y-auto custom-scrollbar p-4">
              <div className="max-w-5xl mx-auto space-y-4">
                {/* Function Name */}
                <div>
                  <label className="block text-xs font-semibold text-[#dfe1e5] mb-2 uppercase tracking-wider">
                    Function Name
                  </label>
                  <input
                    type="text"
                    value={editedName}
                    onChange={(e) => setEditedName(e.target.value)}
                    placeholder="e.g., calculate_tax, format_date"
                    className="w-full px-4 py-2.5 bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)] text-[#dfe1e5] placeholder-[#6e7073] focus:outline-none focus:border-[#3574f0] transition-colors font-mono text-sm"
                  />
                </div>

                {/* Parameters */}
                <div>
                  <label className="block text-xs font-semibold text-[#dfe1e5] mb-2 uppercase tracking-wider">
                    Parameters (comma-separated)
                  </label>
                  <input
                    type="text"
                    value={editedParams}
                    onChange={(e) => setEditedParams(e.target.value)}
                    placeholder="e.g., price, tax_rate, quantity"
                    className="w-full px-4 py-2.5 bg-[#2b2d30] border border-[#43454a] rounded-[var(--radius)] text-[#dfe1e5] placeholder-[#6e7073] focus:outline-none focus:border-[#3574f0] transition-colors font-mono text-sm"
                  />
                  <p className="text-[10px] text-[#6e7073] mt-1">
                    Leave empty for functions with no parameters
                  </p>
                </div>

                {/* Logic Editor */}
                <div className="flex-1 flex flex-col">
                  <label className="block text-xs font-semibold text-[#dfe1e5] mb-2 uppercase tracking-wider">
                    Logic (JSON Array)
                  </label>
                  <div className="h-[400px] border border-[#43454a] rounded-[var(--radius)] overflow-hidden">
                    <Editor
                      height="100%"
                      defaultLanguage="json"
                      theme="vs-dark"
                      value={editedLogic}
                      onChange={(val) => setEditedLogic(val || '[]')}
                      options={{
                        minimap: { enabled: false },
                        fontSize: 13,
                        fontFamily: "'Fira Code', 'Monaco', 'Courier New', monospace",
                        fontLigatures: false,
                        letterSpacing: 0,
                        lineHeight: 20,
                        fontWeight: 'normal',
                        lineNumbers: 'on',
                        scrollBeyondLastLine: false,
                        automaticLayout: true,
                        padding: { top: 12, bottom: 12 },
                        wordWrap: 'off',
                        smoothScrolling: false,
                        cursorBlinking: 'solid',
                        cursorSmoothCaretAnimation: 'off',
                        disableLayerHinting: true,
                        renderWhitespace: 'selection',
                        stopRenderingLineAfter: -1,
                        bracketPairColorization: { enabled: true },
                        formatOnPaste: true,
                        formatOnType: true,
                      }}
                    />
                  </div>
                  <p className="text-[10px] text-[#6e7073] mt-2">
                    Define the function logic as an array of operations (set, math_op, return, etc.)
                  </p>
                </div>
              </div>
            </div>
          </>
        )}

        {/* Copilot Sidebar */}
        <CopilotSidebar
          isOpen={showCopilot}
          onClose={() => setShowCopilot(false)}
          currentCode={editedLogic}
          codeLanguage="json"
          contextType="function"
          onApplyCode={handleCopilotApply}
        />
      </div>

      {/* AI Generator Modal */}
      <AIGeneratorModal
        isOpen={showAIModal}
        onClose={() => setShowAIModal(false)}
        onUseCode={handleAIGenerated}
        type="function"
      />
    </div>
  );
};
