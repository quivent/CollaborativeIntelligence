import React from 'react';
import { AGENT_CATEGORIES, Category } from '../types/categories';

interface CategoryFilterProps {
  selectedCategories: string[];
  onCategoryToggle: (categoryId: string) => void;
  onClearAll: () => void;
  className?: string;
}

export const CategoryFilter: React.FC<CategoryFilterProps> = ({
  selectedCategories,
  onCategoryToggle,
  onClearAll,
  className = ''
}) => {
  const isCompact = className.includes('compact');
  const isGridView = className.includes('grid-view');
  const isDirectoryMain = className.includes('directory-main');
  
  return (
    <div className={`category-filter ${className}`}>
      {selectedCategories.length > 0 && (
        <div className={`flex justify-end ${isCompact ? 'mb-2' : 'mb-4'}`}>
          <button
            onClick={onClearAll}
            className="text-xs text-gray-500 hover:text-gray-700 underline"
          >
            Clear
          </button>
        </div>
      )}
      
      <div className={`grid gap-2 ${
        isCompact 
          ? 'grid-cols-3 gap-1' 
          : isDirectoryMain
          ? 'grid-cols-2 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8'
          : 'grid-cols-2 md:grid-cols-4 lg:grid-cols-8'
      }`}>
        {AGENT_CATEGORIES.map((category) => {
          const isSelected = selectedCategories.includes(category.id);
          return (
            <button
              key={category.id}
              onClick={() => onCategoryToggle(category.id)}
              className={`
                ${isCompact 
                  ? 'px-2 py-1 text-xs min-h-[1.5rem]' 
                  : isDirectoryMain
                  ? 'px-3 py-2 text-sm min-h-[2.25rem]'
                  : 'px-4 py-2 text-sm min-h-[2.5rem]'
                } 
                rounded-lg font-medium transition-all duration-200
                border text-center flex items-center justify-center
                ${isSelected 
                  ? (isGridView || isCompact || isDirectoryMain)
                    ? `${category.color} border-transparent ${isCompact ? 'shadow-sm' : 'shadow-md scale-105'}`
                    : `${category.color} border-transparent shadow-md scale-105`
                  : isCompact 
                    ? 'bg-white text-gray-600 border-gray-200 hover:border-gray-300 hover:bg-gray-50'
                    : 'bg-white text-gray-600 border-gray-200 hover:border-gray-300 hover:shadow-sm hover:bg-gray-50'
                }
              `}
              title={category.description}
            >
              {category.name}
            </button>
          );
        })}
      </div>
      
      {selectedCategories.length > 0 && !isCompact && (
        <div className="mt-3 text-sm text-gray-600">
          Filtering by: {selectedCategories.map(id => 
            AGENT_CATEGORIES.find(c => c.id === id)?.name
          ).join(', ')}
        </div>
      )}
    </div>
  );
};