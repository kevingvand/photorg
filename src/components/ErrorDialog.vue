<template>
  <Teleport to="body">
    <div v-if="isVisible" class="error-dialog-overlay" @click.self="closeDialog">
      <div class="error-dialog">
        <div class="error-header">
          <span class="error-icon">❌</span>
          <h2>{{ title }}</h2>
          <button class="close-btn" @click="closeDialog">×</button>
        </div>

        <div class="error-body">
          <p class="error-description">{{ description }}</p>

          <div class="error-actions">
            <button
              v-for="action in actions"
              :key="action.id"
              class="action-btn"
              @click="handleAction(action.id)"
            >
              {{ action.label }}
            </button>
          </div>

          <details v-if="showDetails" class="error-details">
            <summary>Technical Details</summary>
            <div class="details-content">
              <p v-if="errorCode">
                <strong>Error Code:</strong> {{ errorCode }}
              </p>
              <p v-if="errorMessage">
                <strong>Message:</strong> {{ errorMessage }}
              </p>
              <button
                class="copy-btn"
                @click="copyDetailsToClipboard"
              >
                Copy Details
              </button>
            </div>
          </details>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface ErrorAction {
  id: string
  label: string
  handler: () => void | Promise<void>
}

interface ErrorDialogProps {
  title?: string
  description?: string
  errorCode?: string
  errorMessage?: string
  actions?: ErrorAction[]
  showDetails?: boolean
}

const props = withDefaults(defineProps<ErrorDialogProps>(), {
  title: 'An Error Occurred',
  description: 'Something unexpected happened. Please try again.',
  showDetails: true,
  actions: () => [
    {
      id: 'close',
      label: 'Close',
      handler: () => {},
    },
  ],
})

const isVisible = ref(true)

const closeDialog = () => {
  isVisible.value = false
}

const handleAction = async (actionId: string) => {
  const action = props.actions?.find(a => a.id === actionId)
  if (action) {
    await action.handler()
  }
  closeDialog()
}

const copyDetailsToClipboard = async () => {
  const details = `Error Code: ${props.errorCode || 'N/A'}\nMessage: ${props.errorMessage || 'N/A'}`
  try {
    await navigator.clipboard.writeText(details)
  } catch (err) {
    console.error('Failed to copy to clipboard:', err)
  }
}
</script>

<style scoped>
.error-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.error-dialog {
  background: white;
  border-radius: 8px;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
  max-width: 500px;
  width: 100%;
  animation: slideIn 0.3s ease-out;
}

@keyframes slideIn {
  from {
    transform: translateY(-50px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.error-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  border-bottom: 1px solid #e5e7eb;
}

.error-icon {
  font-size: 1.5rem;
}

.error-header h2 {
  flex: 1;
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: #1f2937;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #6b7280;
  padding: 0;
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.close-btn:hover {
  background-color: #f3f4f6;
}

.error-body {
  padding: 1.5rem;
}

.error-description {
  margin: 0 0 1.5rem 0;
  color: #374151;
  line-height: 1.5;
}

.error-actions {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 1rem;
  flex-wrap: wrap;
}

.action-btn {
  flex: 1;
  min-width: 120px;
  padding: 0.625rem 1rem;
  background-color: #3b82f6;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.action-btn:hover {
  background-color: #2563eb;
}

.action-btn:active {
  background-color: #1d4ed8;
}

.error-details {
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  padding: 0.75rem;
}

.error-details summary {
  cursor: pointer;
  font-weight: 500;
  color: #4b5563;
  padding: 0.25rem;
  user-select: none;
}

.error-details summary:hover {
  color: #1f2937;
}

.details-content {
  margin-top: 0.75rem;
  padding-top: 0.75rem;
  border-top: 1px solid #e5e7eb;
  font-size: 0.875rem;
  color: #6b7280;
}

.details-content p {
  margin: 0.5rem 0;
  word-break: break-word;
}

.copy-btn {
  margin-top: 0.75rem;
  padding: 0.375rem 0.75rem;
  background-color: #f3f4f6;
  color: #374151;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 0.75rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.copy-btn:hover {
  background-color: #e5e7eb;
}

@media (max-width: 640px) {
  .error-dialog {
    margin: 1rem;
  }

  .action-btn {
    min-width: 100px;
  }
}
</style>
