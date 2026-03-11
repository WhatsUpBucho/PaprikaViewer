import { convertFileSrc } from '@tauri-apps/api/core';
import type { RecipeSummary } from '../types';

interface RecipeCardProps {
  recipe: RecipeSummary;
  onClick: () => void;
}

function StarRating({ rating }: { rating: number }) {
  return (
    <div className="recipe-card-stars">
      {[1, 2, 3, 4, 5].map((n) => (
        <span key={n} className={`star${n <= rating ? ' filled' : ''}`}>★</span>
      ))}
    </div>
  );
}

export function RecipeCard({ recipe, onClick }: RecipeCardProps) {
  const photoSrc = recipe.photoCachedPath
    ? convertFileSrc(recipe.photoCachedPath)
    : null;

  return (
    <div className="recipe-card" onClick={onClick} role="button" tabIndex={0}
      onKeyDown={(e) => e.key === 'Enter' && onClick()}
    >
      {photoSrc ? (
        <img
          className="recipe-card-photo"
          src={photoSrc}
          alt={recipe.name}
          loading="lazy"
          onError={(e) => {
            // Fall back to placeholder on error
            (e.target as HTMLElement).style.display = 'none';
          }}
        />
      ) : (
        <div className="recipe-card-photo-placeholder">🍽</div>
      )}
      <div className="recipe-card-body">
        <div className="recipe-card-name" title={recipe.name}>{recipe.name}</div>
        {recipe.rating > 0 && <StarRating rating={recipe.rating} />}
      </div>
    </div>
  );
}
