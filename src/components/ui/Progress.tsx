import React from 'react'
import { cn } from '@/utils/helpers'

interface ProgressProps {
  value: number
  max?: number
  size?: 'sm' | 'md' | 'lg'
  color?: 'primary' | 'secondary' | 'accent'
  showLabel?: boolean
  className?: string
}

export const Progress: React.FC<ProgressProps> = ({
  value,
  max = 100,
  size = 'md',
  color = 'primary',
  showLabel = false,
  className,
}) => {
  const percentage = Math.min(Math.max((value / max) * 100, 0), 100)
  
  const sizeClasses = {
    sm: 'h-1.5',
    md: 'h-2.5',
    lg: 'h-4',
  }
  
  const colorClasses = {
    primary: 'bg-primary',
    secondary: 'bg-secondary',
    accent: 'bg-accent',
  }
  
  return (
    <div className={cn('w-full', className)}>
      <div className={cn('w-full bg-border rounded-full overflow-hidden', sizeClasses[size])}>
        <div
          className={cn('transition-all duration-300 ease-out rounded-full', colorClasses[color], sizeClasses[size])}
          style={{ width: `${percentage}%` }}
          role="progressbar"
          aria-valuenow={value}
          aria-valuemin={0}
          aria-valuemax={max}
        />
      </div>
      {showLabel && (
        <p className="text-sm text-text-secondary mt-1 text-right">
          {Math.round(percentage)}%
        </p>
      )}
    </div>
  )
}
