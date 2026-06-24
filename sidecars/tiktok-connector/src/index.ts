import { program } from 'commander';
import { TikTokClient } from './client';
import * as readline from 'readline';

program
    .requiredOption('-u, --username <username>', 'TikTok Username')
    .requiredOption('-s, --session-id <sessionId>', 'Internal Session ID');

program.parse(process.argv);
const options = program.opts();

const client = new TikTokClient(options.username, options.sessionId);

// Listen to stdin for commands
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

rl.on('line', (line: string) => {
    try {
        const cmd = JSON.parse(line);
        if (cmd.type === 'command' && cmd.command === 'stop') {
            client.stop();
        }
    } catch (e) {
        // ignore invalid json on stdin
    }
});

// Handle kill signals
process.on('SIGINT', () => client.stop());
process.on('SIGTERM', () => client.stop());

client.connect();
