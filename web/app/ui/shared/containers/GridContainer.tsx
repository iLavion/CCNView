// app/ui/shared/containers/GridContainer.tsx

interface GridContainerProps {
  children?: React.ReactNode;
  cols?: number | { xs?: number; sm?: number; md?: number; lg?: number; xl?: number; xxl?: number };
  rows?: number | { xs?: number; sm?: number; md?: number; lg?: number; xl?: number; xxl?: number };
  gap?: number;
  className?: string;
}

export function GridContainer({
  children,
  cols = 12,
  rows = 8,
  gap = 4,
  className = '',
}: GridContainerProps) {
  // Convert simple number to object
  const gridCols = typeof cols === 'number' ? { xs: cols } : cols;
  const gridRows = rows && (typeof rows === 'number' ? { xs: rows } : rows);

  // Build column classes
  let gridClassName = gridCols.xs ? `grid-cols-${gridCols.xs}` : 'grid-cols-12';
  if (gridCols.sm) gridClassName += ` sm:grid-cols-${gridCols.sm}`;
  if (gridCols.md) gridClassName += ` md:grid-cols-${gridCols.md}`;
  if (gridCols.lg) gridClassName += ` lg:grid-cols-${gridCols.lg}`;
  if (gridCols.xl) gridClassName += ` xl:grid-cols-${gridCols.xl}`;
  if (gridCols.xxl) gridClassName += ` 2xl:grid-cols-${gridCols.xxl}`;

  // Build row classes if rows are specified
  if (gridRows) {
    if (gridRows.xs) gridClassName += ` grid-rows-${gridRows.xs}`;
    if (gridRows.sm) gridClassName += ` sm:grid-rows-${gridRows.sm}`;
    if (gridRows.md) gridClassName += ` md:grid-rows-${gridRows.md}`;
    if (gridRows.lg) gridClassName += ` lg:grid-rows-${gridRows.lg}`;
    if (gridRows.xl) gridClassName += ` xl:grid-rows-${gridRows.xl}`;
    if (gridRows.xxl) gridClassName += ` 2xl:grid-rows-${gridRows.xxl}`;
  }

  return (
    <div className={`grid ${gridClassName} gap-${gap} h-full w-full ${className}`}>{children}</div>
  );
}

interface GridItemProps {
  children?: React.ReactNode; // Added question mark to make it optional
  cols?: number | { xs?: number; sm?: number; md?: number; lg?: number; xl?: number; xxl?: number };
  rows?: number | { xs?: number; sm?: number; md?: number; lg?: number; xl?: number; xxl?: number };
}

export function GridItem({ children, cols = { xs: 12, md: 4, lg: 6 }, rows = 1 }: GridItemProps) {
  // Convert simple number to object
  const colSpans = typeof cols === 'number' ? { xs: cols } : cols;
  const rowSpans = typeof rows === 'number' ? { xs: rows } : rows;

  // Build column classes
  let className = colSpans.xs ? `col-span-${colSpans.xs}` : 'col-span-12';
  if (colSpans.sm) className += ` sm:col-span-${colSpans.sm}`;
  if (colSpans.md) className += ` md:col-span-${colSpans.md}`;
  if (colSpans.lg) className += ` lg:col-span-${colSpans.lg}`;
  if (colSpans.xl) className += ` xl:col-span-${colSpans.xl}`;
  if (colSpans.xxl) className += ` 2xl:col-span-${colSpans.xxl}`;

  // Add row span classes
  if (rowSpans.xs) className += ` row-span-${rowSpans.xs}`;
  if (rowSpans.sm) className += ` sm:row-span-${rowSpans.sm}`;
  if (rowSpans.md) className += ` md:row-span-${rowSpans.md}`;
  if (rowSpans.lg) className += ` lg:row-span-${rowSpans.lg}`;
  if (rowSpans.xl) className += ` xl:row-span-${rowSpans.xl}`;
  if (rowSpans.xxl) className += ` 2xl:row-span-${rowSpans.xxl}`;

  return <div className={`${className}`}>{children}</div>;
}
