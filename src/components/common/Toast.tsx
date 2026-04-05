import { create } from 'zustand'

export interface Toast {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  title: string
  message?: string
  duration?: number
}

interface ToastStore {
  toasts: Toast[]
  addToast: (toast: Omit<Toast, 'id'>) => void
  removeToast: (id: string) => void
}

export const useToastStore = create<ToastStore>((set) => ({
  toasts: [],
  addToast: (toast) => {
    const id = Date.now().toString()
    set((state) => ({ toasts: [...state.toasts, { ...toast, id }] }))

    // Auto-remove after duration
    const duration = toast.duration ?? 5000
    if (duration > 0) {
      setTimeout(() => {
        set((state) => ({ toasts: state.toasts.filter((t) => t.id !== id) }))
      }, duration)
    }
  },
  removeToast: (id) => set((state) => ({ toasts: state.toasts.filter((t) => t.id !== id) })),
}))

export function toastSuccess(title: string, message?: string) {
  useToastStore.getState().addToast({ type: 'success', title, message })
}

export function toastError(title: string, message?: string) {
  useToastStore.getState().addToast({ type: 'error', title, message })
}

export function toastWarning(title: string, message?: string) {
  useToastStore.getState().addToast({ type: 'warning', title, message })
}

export function toastInfo(title: string, message?: string) {
  useToastStore.getState().addToast({ type: 'info', title, message })
}
