import { program } from 'commander';
import { TikTokClient } from './client';
import { setupStdinListener } from './stdin';

program
    .requiredOption('-u, --username <username>', 'TikTok Username')
    .requiredOption('-s, --session-id <sessionId>', 'Internal Session ID');

program.parse(process.argv);
const options = program.opts();

const client = new TikTokClient(options.username, options.sessionId);

// Listen to stdin for commands
setupStdinListener(client);

// Handle kill signals
process.on('SIGINT', () => client.stop());
process.on('SIGTERM', () => client.stop());

client.connect();
