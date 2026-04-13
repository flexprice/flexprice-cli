import dotenv from 'dotenv';

dotenv.config();

import { mainError, run } from './main.js';

try {
  await run(process.argv);
} catch (err) {
  mainError(err);
  process.exit(1);
}
