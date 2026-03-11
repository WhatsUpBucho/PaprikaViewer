import { invoke } from '@tauri-apps/api/core';
import type { RecipeSummary, RecipeDetail, CategoryInfo, RecipeFilters } from '../types';

export const api = {
  login: (email: string, password: string) =>
    invoke<void>('login', { email, password }),

  logout: () =>
    invoke<void>('logout'),

  checkAuth: () =>
    invoke<boolean>('check_auth'),

  syncRecipes: () =>
    invoke<number>('sync_recipes'),

  getRecipes: (filters: RecipeFilters = {}) =>
    invoke<RecipeSummary[]>('get_recipes', { filters }),

  getRecipeDetail: (uid: string) =>
    invoke<RecipeDetail>('get_recipe_detail', { uid }),

  getCategories: () =>
    invoke<CategoryInfo[]>('get_categories'),
};
