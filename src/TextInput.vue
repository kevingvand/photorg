<script setup lang="ts">
import { ref } from 'vue';

interface Props {
  modelValue?: string;
  placeholder?: string;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  placeholder: 'Enter text...',
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'submit': [value: string];
}>();

const input = ref(props.modelValue);

const handleSubmit = () => {
  emit('submit', input.value);
};

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  input.value = target.value;
  emit('update:modelValue', input.value);
};
</script>

<template>
  <div class="text-input">
    <input
      :value="input"
      :placeholder="placeholder"
      @input="handleInput"
      @keydown.enter="handleSubmit"
      data-testid="text-input"
    />
    <button @click="handleSubmit" data-testid="submit-button">Submit</button>
  </div>
</template>

<style scoped>
.text-input {
  display: flex;
  gap: 8px;
}

input {
  flex: 1;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

button {
  padding: 8px 16px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #0056b3;
}
</style>
