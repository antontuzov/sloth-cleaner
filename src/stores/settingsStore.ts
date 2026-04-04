import { create } from 'zustand'

export type Theme = 'light' | 'dark' | 'system'

interface SettingsState {
  theme: Theme
  language: string
  autoStart: boolean
  enableRollback: boolean
  dryRunAlways: boolean
  minFileAgeDays: number
  enableLearning: boolean
  aggressiveness: number
  protectedPaths: string[]
  
  setTheme: (theme: Theme) => void
  setLanguage: (language: string) => void
  setAutoStart: (autoStart: boolean) => void
  setEnableRollback: (enable: boolean) => void
  setDryRunAlways: (enabled: boolean) => void
  setMinFileAgeDays: (days: number) => void
  setEnableLearning: (enabled: boolean) => void
  setAggressiveness: (level: number) => void
  addProtectedPath: (path: string) => void
  removeProtectedPath: (path: string) => void
  resetLearning: () => void
}

export const useSettingsStore = create<SettingsState>((set) => ({
  theme: 'system',
  language: 'en',
  autoStart: false,
  enableRollback: true,
  dryRunAlways: true,
  minFileAgeDays: 1,
  enableLearning: true,
  aggressiveness: 3,
  protectedPaths: [],
  
  setTheme: (theme) => set({ theme }),
  setLanguage: (language) => set({ language }),
  setAutoStart: (autoStart) => set({ autoStart }),
  setEnableRollback: (enable) => set({ enableRollback: enable }),
  setDryRunAlways: (enabled) => set({ dryRunAlways: enabled }),
  setMinFileAgeDays: (days) => set({ minFileAgeDays: days }),
  setEnableLearning: (enabled) => set({ enableLearning: enabled }),
  setAggressiveness: (level) => set({ aggressiveness: level }),
  addProtectedPath: (path) => set((state) => ({ protectedPaths: [...state.protectedPaths, path] })),
  removeProtectedPath: (path) => set((state) => ({ protectedPaths: state.protectedPaths.filter((p) => p !== path) })),
  resetLearning: () => set({}),
}))
