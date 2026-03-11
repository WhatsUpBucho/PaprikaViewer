import { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';
import { convertFileSrc } from '@tauri-apps/api/core';
import { api } from '../lib/tauri';
import type { RecipeDetail } from '../types';

interface RecipeDetailModalProps {
  uid: string;
  onClose: () => void;
}

function MetaItem({ label, value }: { label: string; value: string }) {
  if (!value) return null;
  return (
    <div className="meta-item">
      <span className="meta-label">{label}</span>
      <span className="meta-value">{value}</span>
    </div>
  );
}

function StarRating({ rating }: { rating: number }) {
  if (rating === 0) return null;
  return (
    <div className="meta-item">
      <span className="meta-label">Rating</span>
      <span className="meta-value">
        {[1,2,3,4,5].map((n) => (
          <span key={n} style={{ color: n <= rating ? '#f5a623' : '#d9cfc7' }}>★</span>
        ))}
      </span>
    </div>
  );
}

export function RecipeDetailModal({ uid, onClose }: RecipeDetailModalProps) {
  const [recipe, setRecipe] = useState<RecipeDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    setLoading(true);
    setError('');
    api.getRecipeDetail(uid)
      .then(setRecipe)
      .catch(() => setError('Failed to load recipe.'))
      .finally(() => setLoading(false));
  }, [uid]);

  // Close on Escape
  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if (e.key === 'Escape') onClose();
    }
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [onClose]);

  const photoSrc = recipe?.photoCachedPath
    ? convertFileSrc(recipe.photoCachedPath)
    : null;

  const modalContent = (
    <div className="modal-overlay" onClick={(e) => e.target === e.currentTarget && onClose()}>
      <div className="modal-content" role="dialog" aria-modal>
        {loading && (
          <div style={{ padding: 48, textAlign: 'center', color: 'var(--text-muted)' }}>
            Loading…
          </div>
        )}
        {error && (
          <div style={{ padding: 32, textAlign: 'center', color: '#c0392b' }}>{error}</div>
        )}
        {recipe && (
          <>
            <div className="modal-header">
              {photoSrc && (
                <img className="modal-photo" src={photoSrc} alt={recipe.name} />
              )}
              <h2 className="modal-title">{recipe.name}</h2>
              <div className="modal-header-actions">
                <button className="btn-print" onClick={() => window.print()}>
                  🖨 Print Recipe
                </button>
                <button className="modal-close" onClick={onClose} aria-label="Close">×</button>
              </div>
            </div>

            <div className="modal-body">
              {recipe.description && (
                <p className="description-text" style={{ marginBottom: 16 }}>{recipe.description}</p>
              )}

              {recipe.categories.length > 0 && (
                <div className="modal-categories">
                  {recipe.categories.map((cat) => (
                    <span key={cat} className="category-tag">{cat}</span>
                  ))}
                </div>
              )}

              <div className="modal-meta">
                <MetaItem label="Servings" value={recipe.servings} />
                <MetaItem label="Prep Time" value={recipe.prepTime} />
                <MetaItem label="Cook Time" value={recipe.cookTime} />
                <MetaItem label="Total Time" value={recipe.totalTime} />
                <MetaItem label="Difficulty" value={recipe.difficulty} />
                <StarRating rating={recipe.rating} />
              </div>

              {recipe.ingredients && (
                <>
                  <div className="section-heading">Ingredients</div>
                  <pre className="ingredients-text">{recipe.ingredients}</pre>
                </>
              )}

              {recipe.directions && (
                <>
                  <div className="section-heading">Directions</div>
                  <pre className="directions-text">{recipe.directions}</pre>
                </>
              )}

              {recipe.notes && (
                <>
                  <div className="section-heading">Notes</div>
                  <pre className="notes-text">{recipe.notes}</pre>
                </>
              )}

              {recipe.nutritionalInfo && (
                <>
                  <div className="section-heading">Nutritional Info</div>
                  <pre className="nutrition-text">{recipe.nutritionalInfo}</pre>
                </>
              )}

              {recipe.source && (
                <div style={{ marginTop: 20, fontSize: 12, color: 'var(--text-muted)' }}>
                  Source:{' '}
                  {recipe.sourceUrl ? (
                    <a href={recipe.sourceUrl} target="_blank" rel="noreferrer" className="source-link">
                      {recipe.source}
                    </a>
                  ) : (
                    recipe.source
                  )}
                </div>
              )}

            </div>

            {/* Hidden print view — only visible in @media print */}
            <div className="recipe-print-view">
              {photoSrc && (
                <img className="recipe-print-photo" src={photoSrc} alt={recipe.name} />
              )}
              <h1 className="recipe-print-title">{recipe.name}</h1>
              {recipe.categories.length > 0 && (
                <p className="recipe-print-categories">{recipe.categories.join(' · ')}</p>
              )}
              {recipe.description && (
                <p className="recipe-print-description">{recipe.description}</p>
              )}

              <div className="recipe-print-meta">
                {recipe.servings && (
                  <div className="recipe-print-meta-item">
                    <span className="recipe-print-meta-label">Servings</span>
                    <span className="recipe-print-meta-value">{recipe.servings}</span>
                  </div>
                )}
                {recipe.prepTime && (
                  <div className="recipe-print-meta-item">
                    <span className="recipe-print-meta-label">Prep Time</span>
                    <span className="recipe-print-meta-value">{recipe.prepTime}</span>
                  </div>
                )}
                {recipe.cookTime && (
                  <div className="recipe-print-meta-item">
                    <span className="recipe-print-meta-label">Cook Time</span>
                    <span className="recipe-print-meta-value">{recipe.cookTime}</span>
                  </div>
                )}
                {recipe.totalTime && (
                  <div className="recipe-print-meta-item">
                    <span className="recipe-print-meta-label">Total Time</span>
                    <span className="recipe-print-meta-value">{recipe.totalTime}</span>
                  </div>
                )}
              </div>

              {recipe.ingredients && (
                <>
                  <div className="recipe-print-section-title">Ingredients</div>
                  <pre className="recipe-print-text">{recipe.ingredients}</pre>
                </>
              )}

              {recipe.directions && (
                <>
                  <div className="recipe-print-section-title">Directions</div>
                  <pre className="recipe-print-text">{recipe.directions}</pre>
                </>
              )}

              {recipe.notes && (
                <>
                  <div className="recipe-print-section-title">Notes</div>
                  <pre className="recipe-print-notes">{recipe.notes}</pre>
                </>
              )}

              {(recipe.source || recipe.sourceUrl) && (
                <div className="recipe-print-source">
                  Source: {recipe.sourceUrl || recipe.source}
                </div>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );

  return ReactDOM.createPortal(modalContent, document.body);
}
