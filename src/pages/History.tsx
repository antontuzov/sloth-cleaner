import { useState } from 'react'
import { Card } from '@/components/ui/Card'
import { Button } from '@/components/ui/Button'
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts'

const MOCK_HISTORY_DATA = [
  { date: '2026-03-28', spaceFreed: 2.5 },
  { date: '2026-03-30', spaceFreed: 1.8 },
  { date: '2026-04-01', spaceFreed: 3.2 },
  { date: '2026-04-03', spaceFreed: 2.1 },
]

const MOCK_CLEANUPS = [
  { id: '1', date: '2026-04-03', spaceFreed: 2.1, filesDeleted: 1234, status: 'completed' },
  { id: '2', date: '2026-04-01', spaceFreed: 3.2, filesDeleted: 2345, status: 'completed' },
  { id: '3', date: '2026-03-30', spaceFreed: 1.8, filesDeleted: 876, status: 'completed' },
]

export const History = () => {
  const [snapshots] = useState([
    { id: 'snap-1', date: '2026-04-03', files: 1234, size: 2.1 },
    { id: 'snap-2', date: '2026-04-01', files: 2345, size: 3.2 },
  ])
  
  return (
    <div className="max-w-6xl mx-auto space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-text">History & Analytics</h1>
        <p className="text-text-secondary mt-1">Track your cleanup progress over time</p>
      </div>
      
      {/* Space Saved Chart */}
      <Card title="Space Saved Over Time" subtitle="Last 30 days">
        <div className="h-64">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={MOCK_HISTORY_DATA}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="date" />
              <YAxis label={{ value: 'GB', angle: -90, position: 'insideLeft' }} />
              <Tooltip />
              <Line type="monotone" dataKey="spaceFreed" stroke="#D97757" strokeWidth={2} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </Card>
      
      {/* Cleanup History */}
      <Card title="Recent Cleanups">
        <div className="space-y-3">
          {MOCK_CLEANUPS.map((cleanup) => (
            <div key={cleanup.id} className="flex items-center justify-between p-3 rounded-lg bg-background">
              <div>
                <p className="font-medium">{cleanup.date}</p>
                <p className="text-sm text-text-secondary">{cleanup.filesDeleted.toLocaleString()} files deleted</p>
              </div>
              <div className="text-right">
                <p className="font-semibold text-primary">{cleanup.spaceFreed} GB freed</p>
                <span className="badge badge-success">{cleanup.status}</span>
              </div>
            </div>
          ))}
        </div>
      </Card>
      
      {/* Snapshots */}
      <Card title="Rollback Snapshots">
        <div className="space-y-3">
          {snapshots.map((snapshot) => (
            <div key={snapshot.id} className="flex items-center justify-between p-3 rounded-lg bg-background">
              <div>
                <p className="font-medium">Snapshot {snapshot.id}</p>
                <p className="text-sm text-text-secondary">{snapshot.date} • {snapshot.files} files</p>
              </div>
              <div className="flex items-center gap-4">
                <span className="text-sm">{snapshot.size} GB</span>
                <Button variant="secondary" size="sm">Restore</Button>
              </div>
            </div>
          ))}
        </div>
      </Card>
      
      {/* Summary */}
      <Card>
        <div className="grid grid-cols-3 gap-6 text-center">
          <div>
            <p className="text-3xl font-bold text-primary">9.6 GB</p>
            <p className="text-sm text-text-secondary mt-1">Total freed this month</p>
          </div>
          <div>
            <p className="text-3xl font-bold text-secondary">4</p>
            <p className="text-sm text-text-secondary mt-1">Cleanups performed</p>
          </div>
          <div>
            <p className="text-3xl font-bold text-accent">2.4 GB</p>
            <p className="text-sm text-text-secondary mt-1">Average per cleanup</p>
          </div>
        </div>
      </Card>
    </div>
  )
}
