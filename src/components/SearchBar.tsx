import { useState, useRef, useEffect } from 'react';

interface SearchBarProps {
  value: string;
  onChange: (value: string) => void;
  allNames: string[];
}

export function SearchBar({ value, onChange, allNames }: SearchBarProps) {
  const [showDropdown, setShowDropdown] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(-1);
  const wrapperRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const suggestions = value.trim().length > 0
    ? allNames
        .filter((name) => name.toLowerCase().includes(value.toLowerCase()))
        .slice(0, 8)
    : [];

  // Close dropdown on outside click
  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (wrapperRef.current && !wrapperRef.current.contains(e.target as Node)) {
        setShowDropdown(false);
      }
    }
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
    onChange(e.target.value);
    setShowDropdown(true);
    setHighlightedIndex(-1);
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (!showDropdown || suggestions.length === 0) return;

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setHighlightedIndex((i) => Math.min(i + 1, suggestions.length - 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setHighlightedIndex((i) => Math.max(i - 1, -1));
    } else if (e.key === 'Enter' && highlightedIndex >= 0) {
      e.preventDefault();
      onChange(suggestions[highlightedIndex]);
      setShowDropdown(false);
    } else if (e.key === 'Escape') {
      setShowDropdown(false);
    }
  }

  function handleSuggestionClick(name: string) {
    onChange(name);
    setShowDropdown(false);
    inputRef.current?.focus();
  }

  return (
    <div className="search-wrapper" ref={wrapperRef}>
      <span className="search-icon">
        <svg width="14" height="14" viewBox="0 0 20 20" fill="none" stroke="currentColor" strokeWidth="2">
          <circle cx="9" cy="9" r="7" />
          <path d="m15 15 3.5 3.5" strokeLinecap="round" />
        </svg>
      </span>
      <input
        ref={inputRef}
        type="text"
        className="search-input"
        placeholder="Search recipes…"
        value={value}
        onChange={handleInputChange}
        onFocus={() => value.trim() && setShowDropdown(true)}
        onKeyDown={handleKeyDown}
        autoComplete="off"
        spellCheck={false}
      />
      {value && (
        <button
          className="search-clear"
          onClick={() => { onChange(''); inputRef.current?.focus(); }}
          tabIndex={-1}
          aria-label="Clear search"
        >
          ×
        </button>
      )}
      {showDropdown && suggestions.length > 0 && (
        <div className="autocomplete-dropdown">
          {suggestions.map((name, i) => (
            <div
              key={name}
              className={`autocomplete-item${i === highlightedIndex ? ' highlighted' : ''}`}
              onMouseDown={() => handleSuggestionClick(name)}
            >
              {name}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
