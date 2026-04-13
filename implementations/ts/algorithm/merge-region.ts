interface Region {
  start: number
  end: number
}

interface MergeOptions {
  gap?: number
}

export function mergeRegion(regions: readonly Region[], options: MergeOptions = {}): Region[] {
  if (!regions.length) return []

  // Create a new array and sort regions by start time
  const sortedRegions = [...regions]
    .filter(
      (region): region is Region =>
        typeof region.start === 'number' &&
        typeof region.end === 'number' &&
        region.start <= region.end,
    )
    .sort((a, b) => a.start - b.start)

  if (sortedRegions.length === 0) return []

  const result: Region[] = []
  let current = { ...sortedRegions[0] }
  const gap = options.gap ?? 0

  for (let i = 1; i < sortedRegions.length; i++) {
    const next = sortedRegions[i]

    if (current.end + gap >= next.start) {
      // Merge regions if they overlap or are within the gap
      current.end = Math.max(current.end, next.end)
    } else {
      // No overlap, push current to result and move to next
      result.push({ ...current })
      current = { ...next }
    }
  }

  // Push the last region
  result.push({ ...current })

  return result
}

export default mergeRegion
