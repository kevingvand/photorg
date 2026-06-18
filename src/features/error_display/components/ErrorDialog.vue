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
          <span class="text-2xl">{{ icon }}</span>
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

          <!-- Slot for custom content -->
          <slot />

          <!-- Actions -->
          <div v-if="actions.length > 0" class="mb-4 flex flex-wrap gap-3">
            <button
              v-for="action in actions"
              :key="action.id"
              :class="[
                'flex-1 min-w-[120px] rounded px-4 py-2.5 text-sm font-medium transition-colors',
                action.style === 'secondary'
                  ? 'bg-gray-200 text-gray-900 hover:bg-gray-300 active:bg-gray-400'
                  : 'bg-blue-500 text-white hover:bg-blue-600 active:bg-blue-700',
              ]"
              @click="handleAction(action.id)"
            >
              {{ action.label }}
            </button>
          </div>

          <!-- Details -->
          <details v-if="showDetails && detailsContent" class="rounded border border-gray-200 p-3">
            <summary class="cursor-pointer select-none font-medium text-gray-600 transition-colors hover:text-gray-900 p-1">
              {{ detailsLabel }}
            </summary>
            <div class="mt-3 border-t border-gray-200 pt-3 text-sm text-gray-600">
              <slot name="details">
                <p class="mb-3 break-words whitespace-pre-wrap">{{ detailsContent }}</p>
              </slot>
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
import { ref } from 'vue'

/**
 * Generic reusable dialog component.
 *
 * Can be used for errors, warnings, confirmations, or any other modal content.
 * Supports custom slots for flexible content, multiple action buttons, and details expansion.
 */

interface DialogAction {
  id: string
  label: string
  handler: () => void | Promise<void>
  style?: 'primary' | 'secondary'
}

interface GenericDialogProps {
  title?: string
  description?: string
  icon?: string
  actions?: DialogAction[]
  showDetails?: boolean
  detailsContent?: string
  detailsLabel?: string
}

const props = withDefaults(defineProps<GenericDialogProps>(), {
  title: 'Dialog',
  description: '',
  icon: 'ℹ️',
  actions: () => [
    {
      id: 'close',
      label: 'Close',
      handler: () => {},
      style: 'primary',
    },
  ],
  showDetails: false,
  detailsLabel: 'Details',
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
  const details = props.detailsContent || ''
  try {
    await navigator.clipboard.writeText(details)
  } catch (err) {
    console.error('Failed to copy to clipboard:', err)
  }
}
</script>
