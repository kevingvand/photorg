/**
 * Error Display Feature - Public API
 *
 * This index file exports the public surface of the error_display feature.
 * Only import from this file when using error display functionality.
 *
 * Example:
 * ```ts
 * import { useErrorHandler, type ErrorDetails } from '@/features/error_display'
 * ```
 */

// Types
export type { ErrorDetails, ErrorState, ErrorCategory, ErrorCode, ErrorAction } from './types'
export { ErrorCode, ErrorActionType } from './types'

// Composables
export { useErrorHandler } from './composables/useErrorHandler'

// Components (via async imports if needed)
export { default as GenericDialog } from './components/ErrorDialog.vue'
