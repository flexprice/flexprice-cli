import ora, { type Ora } from 'ora';

export function createSpinner(msg: string): Ora {
  return ora({
    text: msg,
    prefixText: ' ',
    spinner: {
      interval: 80,
      frames: ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏', '✓'],
    },
    color: 'cyan',
  }).start();
}
