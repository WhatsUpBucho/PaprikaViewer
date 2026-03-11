import type { CategoryInfo } from '../types';

interface CategorySidebarProps {
  categories: CategoryInfo[];
  activeCategoryUid: string | null;
  totalCount: number;
  onSelect: (uid: string | null) => void;
}

export function CategorySidebar({
  categories,
  activeCategoryUid,
  totalCount,
  onSelect,
}: CategorySidebarProps) {
  return (
    <nav className="sidebar">
      <div className="sidebar-logo">
        <div className="sidebar-logo-icon">🌶</div>
        <span className="sidebar-logo-text">Paprika Viewer</span>
      </div>

      <div className="sidebar-section-title">Library</div>

      <div
        className={`sidebar-item${activeCategoryUid === null ? ' active' : ''}`}
        onClick={() => onSelect(null)}
      >
        <span>All Recipes</span>
        <span className="sidebar-count">{totalCount}</span>
      </div>

      {categories.length > 0 && (
        <>
          <div className="sidebar-section-title">Categories</div>
          {categories.map((cat) => (
            <div
              key={cat.uid}
              className={`sidebar-item${activeCategoryUid === cat.uid ? ' active' : ''}`}
              onClick={() => onSelect(cat.uid)}
            >
              <span>{cat.name}</span>
              <span className="sidebar-count">{cat.recipeCount}</span>
            </div>
          ))}
        </>
      )}
    </nav>
  );
}
