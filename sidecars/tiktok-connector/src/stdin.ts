import * as readline from 'readline';
import { TikTokClient } from './client';

export function setupStdinListener(client: TikTokClient) {
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
}
