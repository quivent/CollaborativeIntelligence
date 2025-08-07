// Soft, eye-friendly color palette for agent differentiation
export const agentColors = [
  'bg-slate-50 border-slate-200 text-slate-700',
  'bg-blue-50 border-blue-200 text-blue-700',
  'bg-indigo-50 border-indigo-200 text-indigo-700',
  'bg-purple-50 border-purple-200 text-purple-700',
  'bg-pink-50 border-pink-200 text-pink-700',
  'bg-rose-50 border-rose-200 text-rose-700',
  'bg-orange-50 border-orange-200 text-orange-700',
  'bg-amber-50 border-amber-200 text-amber-700',
  'bg-yellow-50 border-yellow-200 text-yellow-700',
  'bg-lime-50 border-lime-200 text-lime-700',
  'bg-green-50 border-green-200 text-green-700',
  'bg-emerald-50 border-emerald-200 text-emerald-700',
  'bg-teal-50 border-teal-200 text-teal-700',
  'bg-cyan-50 border-cyan-200 text-cyan-700',
  'bg-sky-50 border-sky-200 text-sky-700',
  'bg-violet-50 border-violet-200 text-violet-700',
  'bg-fuchsia-50 border-fuchsia-200 text-fuchsia-700',
  'bg-gray-50 border-gray-200 text-gray-700',
  'bg-stone-50 border-stone-200 text-stone-700',
  'bg-zinc-50 border-zinc-200 text-zinc-700',
];

// Generate consistent color for agent based on name
export const getAgentColor = (agentName: string): string => {
  const hash = agentName.split('').reduce((acc, char) => {
    return char.charCodeAt(0) + ((acc << 5) - acc);
  }, 0);
  
  const index = Math.abs(hash) % agentColors.length;
  return agentColors[index];
};

// Hover variants for interactive elements
export const getHoverColor = (colorClass: string): string => {
  return colorClass
    .replace(/bg-(\w+)-50/g, 'hover:bg-$1-100')
    .replace(/border-(\w+)-200/g, 'hover:border-$1-300');
};