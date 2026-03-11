import { useState, useEffect, useCallback, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import { CategorySidebar } from './CategorySidebar';
import { SearchBar } from './SearchBar';
import { RecipeGrid } from './RecipeGrid';
import { RecipeDetailModal } from './RecipeDetailModal';
import { api } from '../lib/tauri';
import type { RecipeSummary, CategoryInfo, SyncProgress } from '../types';

interface MainLayoutProps {
  onLogout: () => void;
}

export function MainLayout({ onLogout }: MainLayoutProps) {
  const [recipes, setRecipes] = useState<RecipeSummary[]>([]);
  const [allNames, setAllNames] = useState<string[]>([]);
  const [categories, setCategories] = useState<CategoryInfo[]>([]);
  const [totalCount, setTotalCount] = useState(0);

  const [searchQuery, setSearchQuery] = useState('');
  const [activeCategoryUid, setActiveCategoryUid] = useState<string | null>(null);
  const [selectedRecipeUid, setSelectedRecipeUid] = useState<string | null>(null);

  const [syncing, setSyncing] = useState(false);
  const [syncStatus, setSyncStatus] = useState('');

  // Debounce search
  const searchTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [debouncedSearch, setDebouncedSearch] = useState('');

  useEffect(() => {
    if (searchTimerRef.current) clearTimeout(searchTimerRef.current);
    searchTimerRef.current = setTimeout(() => setDebouncedSearch(searchQuery), 150);
    return () => {
      if (searchTimerRef.current) clearTimeout(searchTimerRef.current);
    };
  }, [searchQuery]);

  // Fetch recipes whenever filters change
  useEffect(() => {
    api.getRecipes({
      categoryUid: activeCategoryUid ?? undefined,
      searchQuery: debouncedSearch || undefined,
    }).then(setRecipes).catch(console.error);
  }, [activeCategoryUid, debouncedSearch]);

  // Load all recipe names for autocomplete (unfiltered)
  useEffect(() => {
    api.getRecipes({}).then((r) => {
      setAllNames(r.map((x) => x.name));
      setTotalCount(r.length);
    }).catch(console.error);
  }, [recipes]); // refresh names after sync

  // Load categories
  const loadCategories = useCallback(() => {
    api.getCategories().then(setCategories).catch(console.error);
  }, []);

  useEffect(() => {
    loadCategories();
  }, [loadCategories]);

  // Listen for sync progress events
  useEffect(() => {
    const unlisten = listen<SyncProgress>('sync_progress', (event) => {
      const p = event.payload;
      if (p.phase === 'complete') {
        setSyncStatus('');
        setSyncing(false);
        // Refresh after sync
        api.getRecipes({
          categoryUid: activeCategoryUid ?? undefined,
          searchQuery: debouncedSearch || undefined,
        }).then(setRecipes).catch(console.error);
        loadCategories();
      } else {
        const phaseLabel = p.phase === 'entries' ? 'Fetching list'
          : p.phase === 'recipes' ? `Downloading recipes (${p.done}/${p.total})`
          : `Downloading photos (${p.done}/${p.total})`;
        setSyncStatus(phaseLabel);
      }
    });
    return () => { unlisten.then((fn) => fn()); };
  }, [activeCategoryUid, debouncedSearch, loadCategories]);

  async function handleSync() {
    setSyncing(true);
    setSyncStatus('Starting sync…');
    try {
      await api.syncRecipes();
    } catch (err) {
      console.error('Sync failed:', err);
      setSyncStatus('Sync failed');
      setSyncing(false);
    }
  }

  async function handleLogout() {
    await api.logout();
    onLogout();
  }

  function clearFilters() {
    setSearchQuery('');
    setActiveCategoryUid(null);
  }

  const isFiltered = !!searchQuery || activeCategoryUid !== null;

  return (
    <div className="app-layout">
      <CategorySidebar
        categories={categories}
        activeCategoryUid={activeCategoryUid}
        totalCount={totalCount}
        onSelect={setActiveCategoryUid}
      />

      <div style={{ flex: 1, display: 'flex', flexDirection: 'column', minWidth: 0 }}>
        <header className="app-header">
          <SearchBar
            value={searchQuery}
            onChange={setSearchQuery}
            allNames={allNames}
          />

          {isFiltered && (
            <button className="btn-clear" onClick={clearFilters}>
              Clear Filters
            </button>
          )}

          <div style={{ flex: 1 }} />

          {syncStatus && (
            <span className="sync-progress-text">{syncStatus}</span>
          )}

          <button
            className="btn-sync"
            onClick={handleSync}
            disabled={syncing}
          >
            {syncing ? (
              <>
                <span style={{ animation: 'spin 1s linear infinite', display: 'inline-block' }}>↻</span>
                Syncing
              </>
            ) : '↻ Sync'}
          </button>

          <button
            style={{
              background: 'none', border: 'none', cursor: 'pointer',
              color: 'var(--text-muted)', fontSize: 12, padding: '4px 8px',
            }}
            onClick={handleLogout}
            title="Sign out"
          >
            Sign Out
          </button>
        </header>

        <main className="main-area">
          <RecipeGrid
            recipes={recipes}
            onSelectRecipe={setSelectedRecipeUid}
            isFiltered={isFiltered}
          />
        </main>
      </div>

      {selectedRecipeUid && (
        <RecipeDetailModal
          uid={selectedRecipeUid}
          onClose={() => setSelectedRecipeUid(null)}
        />
      )}

      <style>{`
        @keyframes spin { to { transform: rotate(360deg); } }
      `}</style>
    </div>
  );
}
