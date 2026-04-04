import { create } from 'zustand'
import type { ScanResults, ScanProgress } from '@/types'

interface ScannerState {
  scanId: string | null
  progress: ScanProgress | null
  results: ScanResults | null
  isScanning: boolean
  error: string | null
  
  setScanId: (id: string) => void
  setProgress: (progress: ScanProgress) => void
  setResults: (results: ScanResults) => void
  setScanning: (isScanning: boolean) => void
  setError: (error: string | null) => void
  reset: () => void
}

export const useScannerStore = create<ScannerState>((set) => ({
  scanId: null,
  progress: null,
  results: null,
  isScanning: false,
  error: null,
  
  setScanId: (id) => set({ scanId: id }),
  setProgress: (progress) => set({ progress }),
  setResults: (results) => set({ results }),
  setScanning: (isScanning) => set({ isScanning }),
  setError: (error) => set({ error }),
  reset: () => set({ scanId: null, progress: null, results: null, isScanning: false, error: null }),
}))
