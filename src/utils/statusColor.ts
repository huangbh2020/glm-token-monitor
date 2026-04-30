/**
 * Returns a status color based on the used-percentage threshold.
 * Matches the existing PetState color scheme.
 */
export function getStatusColor(percent: number): string {
  if (percent >= 96) return '#6B7280'   // Dead
  if (percent >= 81) return '#F97316'   // Panic
  if (percent >= 61) return '#F59E0B'   // Warning
  return '#3B82F6'                       // Flow / Fresh
}
