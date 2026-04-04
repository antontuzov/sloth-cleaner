import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import { Card } from '@/components/ui/Card'
import { Button } from '@/components/ui/Button'
import { Progress } from '@/components/ui/Progress'
import { formatBytes } from '@/utils/formatBytes'
import { getSystemInfo } from '@/hooks/useScanner'
import type { SystemInfo } from '@/types'
import { HardDrive, Cpu, MemoryStick, Zap, AlertCircle } from 'lucide-react'
import { ROUTES } from '@/utils/constants'

export const Dashboard = () => {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    getSystemInfo()
      .then((info) => {
        setSystemInfo(info)
        setError(null)
      })
      .catch((err) => {
        console.error('Failed to get system info:', err)
        setError('Unable to read system info. The app is running in development mode.')
      })
      .finally(() => setLoading(false))
  }, [])

  if (loading) {
    return <div className="flex items-center justify-center h-64">
      <div className="text-center">
        <div className="animate-spin w-8 h-8 border-4 border-primary border-t-transparent rounded-full mx-auto mb-4"></div>
        <p className="text-text-secondary">Loading system information...</p>
      </div>
    </div>
  }

  const diskUsagePercent = systemInfo
    ? ((systemInfo.diskTotalGb - systemInfo.diskAvailableGb) / systemInfo.diskTotalGb) * 100
    : 0

  // Safe accessor helpers
  const diskAvail = systemInfo?.diskAvailableGb ?? 0
  const diskTotal = systemInfo?.diskTotalGb ?? 0
  const memAvail = systemInfo?.availableMemoryGb ?? 0
  const memTotal = systemInfo?.totalMemoryGb ?? 0
  const cpuBrand = systemInfo?.cpuBrand ?? 'Unknown'
  const cpuCores = systemInfo?.cpuCores ?? 0

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-text">Welcome to SlothCleaner</h1>
          <p className="text-text-secondary mt-1">Clean smart, go slow. Let's optimize your system.</p>
        </div>
        <Link to={ROUTES.scan}>
          <Button size="lg" leftIcon={<Zap className="w-5 h-5" />}>
            Quick Scan
          </Button>
        </Link>
      </div>

      {error && (
        <Card className="border-yellow-200 bg-yellow-50">
          <div className="flex items-start gap-3">
            <AlertCircle className="w-5 h-5 text-yellow-600 mt-0.5" />
            <div>
              <p className="font-medium text-yellow-800">Development Mode</p>
              <p className="text-sm text-yellow-700 mt-1">{error}</p>
            </div>
          </div>
        </Card>
      )}

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card title="Disk Space">
          <div className="space-y-3">
            <div className="flex items-center gap-2">
              <HardDrive className="w-5 h-5 text-primary" />
              <span className="text-2xl font-bold">{formatBytes(diskAvail * 1024 * 1024 * 1024)}</span>
            </div>
            <Progress value={diskUsagePercent} color="primary" showLabel />
            <p className="text-sm text-text-secondary">
              {diskAvail.toFixed(1)} GB available of {diskTotal.toFixed(1)} GB
            </p>
          </div>
        </Card>

        <Card title="Memory">
          <div className="space-y-3">
            <div className="flex items-center gap-2">
              <MemoryStick className="w-5 h-5 text-secondary" />
              <span className="text-2xl font-bold">{memAvail.toFixed(1)} GB</span>
            </div>
            <p className="text-sm text-text-secondary">
              {memTotal.toFixed(1)} GB total
            </p>
          </div>
        </Card>

        <Card title="CPU">
          <div className="space-y-3">
            <div className="flex items-center gap-2">
              <Cpu className="w-5 h-5 text-accent" />
              <span className="text-lg font-semibold truncate">{cpuBrand}</span>
            </div>
            <p className="text-sm text-text-secondary">
              {cpuCores} cores
            </p>
          </div>
        </Card>
      </div>

      {/* AI Recommendation */}
      <Card title="💡 AI Recommendation" subtitle="Based on your system">
        <div className="space-y-4">
          <p className="text-text-secondary">
            Your system has {diskAvail.toFixed(1)} GB available. Running a scan could free up additional space.
          </p>
          <Link to={ROUTES.scan}>
            <Button>Start Scanning</Button>
          </Link>
        </div>
      </Card>

      {/* Recent Activity */}
      <Card title="Recent Activity" subtitle="Your last cleanups will appear here">
        <div className="text-center py-8 text-text-secondary">
          <p>No cleanup history yet</p>
          <p className="text-sm mt-2">Run your first scan to get started!</p>
        </div>
      </Card>
    </div>
  )
}
