import type { RecipeSummary } from '../types';
import { RecipeCard } from './RecipeCard';

interface RecipeGridProps {
  recipes: RecipeSummary[];
  onSelectRecipe: (uid: string) => void;
  isFiltered: boolean;
}

export function RecipeGrid({ recipes, onSelectRecipe, isFiltered }: RecipeGridProps) {
  if (recipes.length === 0) {
    return (
      <div className="empty-state">
        <div className="empty-state-icon">🔍</div>
        <div className="empty-state-title">
          {isFiltered ? 'No matching recipes' : 'No recipes yet'}
        </div>
        <div className="empty-state-sub">
          {isFiltered
            ? 'Try adjusting your search or category filter'
            : 'Click Sync to load your Paprika recipes'}
        </div>
      </div>
    );
  }

  return (
    <>
      <div className="recipe-count">
        {recipes.length} {recipes.length === 1 ? 'recipe' : 'recipes'}
      </div>
      <div className="recipe-grid">
        {recipes.map((recipe) => (
          <RecipeCard
            key={recipe.uid}
            recipe={recipe}
            onClick={() => onSelectRecipe(recipe.uid)}
          />
        ))}
      </div>
    </>
  );
}
