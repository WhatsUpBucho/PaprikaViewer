export interface RecipeSummary {
  uid: string;
  name: string;
  photoCachedPath: string | null;
  rating: number;
  onFavorites: boolean;
  inTrash: boolean;
}

export interface RecipeDetail {
  uid: string;
  name: string;
  photoCachedPath: string | null;
  categories: string[];
  rating: number;
  onFavorites: boolean;
  isPinned: boolean;
  servings: string;
  prepTime: string;
  cookTime: string;
  totalTime: string;
  difficulty: string;
  source: string;
  sourceUrl: string | null;
  ingredients: string;
  directions: string;
  notes: string;
  nutritionalInfo: string;
  created: string;
  description: string;
}

export interface CategoryInfo {
  uid: string;
  name: string;
  orderFlag: number;
  parentUid: string | null;
  recipeCount: number;
}

export interface RecipeFilters {
  categoryUid?: string;
  searchQuery?: string;
  includeTrash?: boolean;
}

export interface SyncProgress {
  total: number;
  done: number;
  phase: 'entries' | 'recipes' | 'photos' | 'complete';
}
