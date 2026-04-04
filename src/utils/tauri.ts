// Tauri API stub - provides invoke() when running outside Tauri context
// In production (tauri dev/build), the real API is injected by Tauri

type Invoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>

// Check if we're running inside Tauri
function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

export const invoke: Invoke = async (command, args) => {
  if (isTauri()) {
    // @ts-ignore - Tauri injects this at runtime
    return window.__TAURI_INTERNALS__.invoke(command, args)
  }
  
  // Fallback for development outside Tauri
  console.warn(`[Tauri Stub] invoke('${command}') called outside Tauri context`)
  throw new Error(`Tauri invoke('${command}') not available outside Tauri runtime`)
}
