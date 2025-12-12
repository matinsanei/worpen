import React from 'react';
import { ResponsiveContainer, LineChart, Line, XAxis, Tooltip } from 'recharts';

interface StatsChartProps {
  data: any[];
  dataKey: string;
  color?: string;
}

export const StatsChart: React.FC<StatsChartProps> = ({ data, dataKey, color = "#00ff41" }) => {
  return (
    <div className="w-full h-full min-h-[100px]">
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={data}>
          <XAxis dataKey="time" hide />
          <Tooltip 
            contentStyle={{ backgroundColor: '#000', border: '1px solid #00ff41', color: '#00ff41', fontFamily: 'monospace' }}
            itemStyle={{ color: color }}
          />
          <Line 
            type="step" 
            dataKey={dataKey} 
            stroke={color} 
            strokeWidth={2} 
            dot={false} 
            activeDot={{ r: 4, fill: color }}
            isAnimationActive={true}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
};