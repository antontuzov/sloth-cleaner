import { cn } from '@/utils/helpers'

interface SkeletonProps {
  className?: string
  variant?: 'text' | 'circular' | 'rectangular'
  width?: string
  height?: string
}

export const Skeleton: React.FC<SkeletonProps> = ({
  className,
  variant = 'rectangular',
  width,
  height,
}) => {
  const variantClasses = {
    text: 'rounded',
    circular: 'rounded-full',
    rectangular: 'rounded-lg',
  }

  return (
    <div
      className={cn(
        'animate-pulse bg-border',
        variantClasses[variant],
        className
      )}
      style={{ width, height }}
      aria-hidden="true"
    />
  )
}

export const CardSkeleton: React.FC = () => (
  <div className="bg-surface rounded-xl border border-border p-6 shadow-sm space-y-4">
    <Skeleton variant="text" width="40%" height="20px" />
    <Skeleton variant="text" width="80%" height="16px" />
    <Skeleton variant="text" width="60%" height="16px" />
  </div>
)

export const PageSkeleton: React.FC = () => (
  <div className="max-w-6xl mx-auto space-y-6">
    <div className="space-y-2">
      <Skeleton variant="text" width="40%" height="28px" />
      <Skeleton variant="text" width="60%" height="16px" />
    </div>
    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
      <CardSkeleton />
      <CardSkeleton />
      <CardSkeleton />
    </div>
    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
      <CardSkeleton />
      <CardSkeleton />
    </div>
  </div>
)
