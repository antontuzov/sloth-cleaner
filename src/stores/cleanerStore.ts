import { create } from 'zustand'
import type { Snapshot } from '@/types'

interface CleanerState {
  selectedCategories: string[]
  isCleaning: boolean
  cleanupProgress: number
  lastCleanupResult: {
    freedSpace: number
    filesDeleted: number
    snapshotId?: string
  } | null
  snapshots: Snapshot[]
  error: string | null
  
  toggleCategory: (category: string) => void
  selectAllSafe: (safeCategories: string[]) => void
  setCleaning: (isCleaning: boolean) => void
  setCleanupProgress: (progress: number) => void
  setLastCleanupResult: (result: { freedSpace: number; filesDeleted: number; snapshotId?: string }) => void
  setSnapshots: (snapshots: Snapshot[]) => void
  setError: (error: string | null) => void
  reset: () => void
}

export const useCleanerStore = create<CleanerState>((set) => ({
  selectedCategories: [],
  isCleaning: false,
  cleanupProgress: 0,
  lastCleanupResult: null,
  snapshots: [],
  error: null,
  
  toggleCategory: (category) => set((state) => ({
    selectedCategories: state.selectedCategories.includes(category)
      ? state.selectedCategories.filter((c) => c !== category)
      : [...state.selectedCategories, category],
  })),
  selectAllSafe: (safeCategories) => set({ selectedCategories: safeCategories }),
  setCleaning: (isCleaning) => set({ isCleaning }),
  setCleanupProgress: (cleanupProgress) => set({ cleanupProgress }),
  setLastCleanupResult: (lastCleanupResult) => set({ lastCleanupResult }),
  setSnapshots: (snapshots) => set({ snapshots }),
  setError: (error) => set({ error }),
  reset: () => set({ selectedCategories: [], isCleaning: false, cleanupProgress: 0, lastCleanupResult: null, error: null }),
}))
