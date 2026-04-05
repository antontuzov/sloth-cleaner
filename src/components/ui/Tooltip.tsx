import { cn } from '@/utils/helpers'

interface TooltipProps {
  content: string
  children: React.ReactNode
  side?: 'top' | 'right' | 'bottom' | 'left'
  delay?: number
  className?: string
}

// Simple tooltip using native title attribute (no external deps)
export const Tooltip: React.FC<TooltipProps> = ({
  content,
  children,
  className,
}) => {
  return (
    <span className={cn('inline-block', className)} title={content}>
      {children}
    </span>
  )
}
