import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import TextInput from './TextInput.vue';

describe('TextInput.vue', () => {
  it('renders with placeholder', () => {
    const wrapper = mount(TextInput, {
      props: {
        placeholder: 'Enter text here...',
      },
    });

    const input = wrapper.find('input');
    expect(input.attributes('placeholder')).toBe('Enter text here...');
  });

  it('updates modelValue on input', async () => {
    const wrapper = mount(TextInput, {
      props: {
        modelValue: '',
      },
    });

    const input = wrapper.find('input');
    await input.setValue('test value');

    expect(wrapper.emitted('update:modelValue')).toBeTruthy();
    expect(wrapper.emitted('update:modelValue')?.[0]).toEqual(['test value']);
  });

  it('emits submit event on button click', async () => {
    const wrapper = mount(TextInput, {
      props: {
        modelValue: 'initial',
      },
    });

    const button = wrapper.find('[data-testid="submit-button"]');
    await button.trigger('click');

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')?.[0]).toEqual(['initial']);
  });

  it('emits submit event on Enter key', async () => {
    const wrapper = mount(TextInput, {
      props: {
        modelValue: '',
      },
    });

    const input = wrapper.find('input');
    await input.setValue('submit me');
    await input.trigger('keydown.enter');

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')?.[0]).toEqual(['submit me']);
  });

  it('handles empty input gracefully', async () => {
    const wrapper = mount(TextInput);

    const button = wrapper.find('[data-testid="submit-button"]');
    await button.trigger('click');

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')?.[0]).toEqual(['']);
  });

  it('respects initial modelValue', async () => {
    const wrapper = mount(TextInput, {
      props: {
        modelValue: 'initial text',
      },
    });

    const input = wrapper.find('input') as any;
    expect(input.element.value).toBe('initial text');
  });
});
