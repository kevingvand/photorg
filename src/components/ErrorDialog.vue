<template>
  <Teleport to="body">
    <div
      v-if="isVisible"
      class="fixed inset-0 flex items-center justify-center bg-black/50 z-1000 p-4"
      @click.self="closeDialog"
    >
      <div class="w-full max-w-md rounded-lg bg-white shadow-lg animate-in slide-in-from-top-2 duration-300">
        <!-- Header -->
        <div class="flex items-center gap-4 border-b border-gray-200 p-6">
          <span class="text-2xl">❌</span>
          <h2 class="flex-1 text-lg font-semibold text-gray-900">{{ title }}</h2>
          <button
            class="flex h-8 w-8 items-center justify-center rounded transition-colors hover:bg-gray-100"
            @click="closeDialog"
          >
            ×
          </button>
        </div>

        <!-- Body -->
        <div class="p-6">
          <p class="mb-6 text-gray-700 leading-relaxed">{{ description }}</p>

          <!-- Actions -->
          <div class="mb-4 flex flex-wrap gap-3">
            <button
              v-for="action in actions"
              :key="action.id"
              class="flex-1 min-w-[120px] rounded bg-blue-500 px-4 py-2.5 text-sm font-medium text-white transition-colors hover:bg-blue-600 active:bg-blue-700"
              @click="handleAction(action.id)"
            >
              {{ action.label }}
            </button>
          </div>

          <!-- Details -->
          <details v-if="showDetails" class="rounded border border-gray-200 p-3">
            <summary class="cursor-pointer select-none font-medium text-gray-600 transition-colors hover:text-gray-900 p-1">
              Technical Details
            </summary>
            <div class="mt-3 border-t border-gray-200 pt-3 text-sm text-gray-600">
              <p v-if="errorCode" class="mb-2">
                <strong>Error Code:</strong> {{ errorCode }}
              </p>
              <p v-if="errorMessage" class="mb-3 break-words">
                <strong>Message:</strong> {{ errorMessage }}
              </p>
              <button
                class="rounded border border-gray-300 bg-gray-100 px-3 py-1.5 text-xs transition-colors hover:bg-gray-200"
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
