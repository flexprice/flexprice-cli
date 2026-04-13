import { input, confirm, number, select } from '@inquirer/prompts';

export { input, confirm, number, select };

/** Prompt for a required string — re-prompts if blank. */
export async function requiredInput(message: string): Promise<string> {
  return input({
    message,
    validate: (v) => (v.trim().length > 0 ? true : 'This field is required'),
  });
}

/** Prompt for an optional string — returns undefined if blank. */
export async function optionalInput(message: string, defaultValue?: string): Promise<string | undefined> {
  const val = await input({ message, default: defaultValue });
  return val.trim().length > 0 ? val.trim() : undefined;
}

/** Convert a display name to a snake_case lookup key suggestion. */
export function toSnakeCase(name: string): string {
  return name
    .toLowerCase()
    .replace(/\s+/g, '_')
    .replace(/[^a-z0-9_]/g, '');
}
