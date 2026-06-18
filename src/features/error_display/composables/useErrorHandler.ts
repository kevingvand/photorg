import { ref, computed } from 'vue'
import type { ErrorDetails, ErrorState, ErrorCategory, ErrorCode } from '../types'

/**
 * Error handler composable for the error display feature.
 *
 * Manages error state and provides utilities for displaying errors to the user.
 * The backend (Rust) determines appropriate UI treatment via ErrorDisplay struct;
 * this composable just tracks state and formats for Vue components.
 */

const currentError = ref<ErrorDetails | null>(null)
const isErrorVisible = ref(false)
const showTechnicalDetails = ref(false)

/**
 * Determine the icon to display based on error category.
 */
function getIconForCategory(category: ErrorCategory): string {
  switch (category) {
    case 'user':
      return '⚠️'
    case 'system':
      return '❌'
    case 'algorithm':
      return '🔧'
    default:
      return 'ℹ️'
  }
}

/**
 * Determine the title to display based on error category.
 */
function getTitleForCategory(category: ErrorCategory): string {
  switch (category) {
    case 'user':
      return 'Unable to Proceed'
    case 'system':
      return 'System Error'
    case 'algorithm':
      return 'Processing Failed'
    default:
      return 'Error'
  }
}

/**
 * Display an error to the user.
 */
export function showError(error: ErrorDetails): void {
  currentError.value = error
  isErrorVisible.value = true
  showTechnicalDetails.value = false

  // Log error to console in development
  if (import.meta.env.DEV) {
    console.error('Photorg Error:', error)
  }
}

/**
 * Clear the current error.
 */
export function clearError(): void {
  currentError.value = null
  isErrorVisible.value = false
  showTechnicalDetails.value = false
}

/**
 * Toggle technical details visibility.
 */
export function toggleDetails(): void {
  showTechnicalDetails.value = !showTechnicalDetails.value
}

/**
 * Get formatted error state for UI components.
 */
const errorState = computed((): ErrorState | null => {
  if (!currentError.value) {
    return null
  }

  const error = currentError.value
  return {
    isVisible: isErrorVisible.value,
    title: getTitleForCategory(error.category),
    message: error.user_message,
    recovery: error.recovery,
    category: error.category,
    code: error.code,
    technicalDetails: error.message,
    showDetails: showTechnicalDetails.value,
  }
})

/**
 * Get icon for the current error.
 */
const icon = computed(() => {
  if (!currentError.value) return 'ℹ️'
  return getIconForCategory(currentError.value.category)
})

/**
 * Send diagnostic report (stub for MVP).
 */
async function sendDiagnosticReport(): Promise<void> {
  try {
    // This would call a Tauri backend command to create the diagnostic report
    // await invoke('create_diagnostic_report', { error: currentError.value })
    console.log('Diagnostic report created (stubbed for MVP)')
  } catch (err) {
    console.error('Failed to create diagnostic report:', err)
  }
}

/**
 * Composable for error handling in the error display feature.
 */
export function useErrorHandler() {
  return {
    // State
    currentError,
    isErrorVisible,
    errorState,
    icon,

    // Methods
    showError,
    clearError,
    toggleDetails,
    sendDiagnosticReport,
  }
}

