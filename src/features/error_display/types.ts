/**
 * Error Display Feature - TypeScript Types
 *
 * This module defines the contract between the backend (Rust) and frontend (Vue)
 * for error display. These types match the backend's Rust structures, ensuring
 * serialization/deserialization compatibility.
 */

/**
 * Error category determines how the frontend treats the error.
 * - "user": User can fix this (invalid input, missing files)
 * - "system": Diagnostic error (disk full, permissions)
 * - "algorithm": Requires investigation (rater failure, conflicts)
 */
export type ErrorCategory = 'user' | 'system' | 'algorithm';

/**
 * Error code identifies the specific error for filtering and tracking.
 */
export enum ErrorCode {
  // User errors
  MissingFile = 'MissingFile',
  InvalidPath = 'InvalidPath',
  UnsupportedFormat = 'UnsupportedFormat',
  InvalidInput = 'InvalidInput',
  FileNotFound = 'FileNotFound',

  // System errors
  PermissionDenied = 'PermissionDenied',
  DiskFull = 'DiskFull',
  DatabaseCorruption = 'DatabaseCorruption',
  OutOfMemory = 'OutOfMemory',
  IoError = 'IoError',

  // Algorithm errors
  UnresolvableConflict = 'UnresolvableConflict',
  RaterFailure = 'RaterFailure',
  DataInvariantViolation = 'DataInvariantViolation',
  PairingFailure = 'PairingFailure',

  // Generic
  Unknown = 'Unknown',
}

/**
 * Complete error details from backend.
 * Sent from Rust to Vue for display.
 */
export interface ErrorDetails {
  category: ErrorCategory;
  code: ErrorCode;
  message: string;
  user_message: string;
  recovery: string;
  context: Record<string, unknown>;
  backtrace?: string;
}

/**
 * Frontend-specific error state for managing UI display.
 */
export interface ErrorState {
  isVisible: boolean;
  title: string;
  message: string;
  recovery: string;
  category: ErrorCategory;
  code: ErrorCode;
  technicalDetails?: string;
  showDetails: boolean;
}

/**
 * Error handler action types for state management.
 */
export enum ErrorActionType {
  ShowError = 'SHOW_ERROR',
  HideError = 'HIDE_ERROR',
  ToggleDetails = 'TOGGLE_DETAILS',
  ClearError = 'CLEAR_ERROR',
}

/**
 * Action dispatched to error handler.
 */
export interface ErrorAction {
  type: ErrorActionType;
  error?: ErrorDetails;
}
