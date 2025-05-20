import React from 'react';

interface PercentageFormatterProps {
  pct: number | null | undefined;
}

export const PercentageFormatter: React.FC<PercentageFormatterProps> = ({ pct }) => {
  if (pct === null || pct === undefined) {
    return null;
  }

  const formattedPct = pct.toFixed(2);

  const color = pct < 0 ? 'red' : pct > 0 ? 'green' : 'black';

  return <div style={{ color }}>{formattedPct}%</div>;
};

export default PercentageFormatter;

