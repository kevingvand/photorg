import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface ErrorDetails {
  category: 'user' | 'system' | 'algorithm'
  code: string
  message: string
  user_message: string
  recovery: string
  context: Record<string, unknown>
  backtrace?: string
}

interface DisplayedError {
  title: string
  description: string
  errorCode: string
  errorMessage: string
  category: ErrorDetails['category']
  actions: Array<{
    id: string
    label: string
    handler: () => void | Promise<void>
  }>
}

const currentError = ref<DisplayedError | null>(null)
const isErrorVisible = ref(false)

/**
 * Format error for UI display based on error category
 */
function formatErrorForUI(error: ErrorDetails): DisplayedError {
  let title = 'An Error Occurred'
  let actions: DisplayedError['actions'] = [
    { id: 'close', label: 'Close', handler: () => {} },
  ]

  switch (error.category) {
    case 'user':
      title = 'Input Error'
      actions = [
        { id: 'close', label: 'Close', handler: () => {} },
        { id: 'retry', label: 'Try Again', handler: () => {} },
      ]
      break

    case 'system':
      title = 'System Error'
      actions = [
        { id: 'close', label: 'Close', handler: () => {} },
        { id: 'retry', label: 'Try Again', handler: () => {} },
        {
          id: 'report',
          label: 'Send Diagnostic Report',
          handler: sendDiagnosticReport,
        },
      ]
      break

    case 'algorithm':
      title = 'Algorithm Error'
      actions = [
        { id: 'close', label: 'Close', handler: () => {} },
        {
          id: 'details',
          label: 'View Details',
          handler: showDetailedInfo,
        },
      ]
      break
  }

  return {
    title,
    description: error.user_message,
    errorCode: error.code,
    errorMessage: error.message,
    category: error.category,
    actions,
  }
}

/**
 * Display an error to the user
 */
export function showError(error: ErrorDetails): void {
  currentError.value = formatErrorForUI(error)
  isErrorVisible.value = true

  // Log error to console in development
  if (import.meta.env.DEV) {
    console.error('Photorg Error:', error)
  }
}

/**
 * Clear the current error
 */
export function clearError(): void {
  currentError.value = null
  isErrorVisible.value = false
}

/**
 * Send diagnostic report (zip logs + error context)
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
 * Show detailed error information
 */
function showDetailedInfo(): void {
  if (currentError.value) {
    console.log('Detailed error info:', currentError.value)
  }
}

/**
 * Composable for error handling
 */
export function useErrorHandler() {
  return {
    currentError,
    isErrorVisible,
    showError,
    clearError,
  }
}
