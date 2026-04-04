import React from 'react'
import * as Dialog from '@radix-ui/react-dialog'
import { X } from 'lucide-react'
import { cn } from '@/utils/helpers'

interface ModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  title?: string
  description?: string
  children: React.ReactNode
  className?: string
}

export const Modal: React.FC<ModalProps> = ({
  open,
  onOpenChange,
  title,
  description,
  children,
  className,
}) => {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50 backdrop-blur-sm data-[state=open]:animate-in data-[state=closed]:animate-out" />
        <Dialog.Content
          className={cn(
            'fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-lg bg-surface rounded-xl border border-border shadow-lg p-6 focus:outline-none',
            className
          )}
        >
          {(title || description) && (
            <div className="mb-4">
              {title && <Dialog.Title className="text-lg font-semibold text-text">{title}</Dialog.Title>}
              {description && <Dialog.Description className="text-sm text-text-secondary">{description}</Dialog.Description>}
            </div>
          )}
          {children}
          <Dialog.Close asChild>
            <button
              className="absolute top-4 right-4 p-1 rounded-lg hover:bg-background transition-colors"
              aria-label="Close"
            >
              <X className="w-4 h-4" />
            </button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  )
}
