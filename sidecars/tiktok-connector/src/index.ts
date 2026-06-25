import { program } from 'commander';
import { TikTokClient } from './client';
import { setupStdinListener } from './stdin';

program
    .requiredOption('-u, --username <username>', 'TikTok Username')
    .requiredOption('-s, --session-id <sessionId>', 'Internal Session ID')
    .option(
        '--tiktok-cookie <cookie>',
        'TikTok session cookie (sessionid=...; tt-target-idc=...) for stream playback',
    );

program.parse(process.argv);
const options = program.opts();
const tiktokCookie =
    options.tiktokCookie ||
    process.env.TIKTOK_SESSION_COOKIE ||
    process.env.TIKTOK_SESSIONID;

const client = new TikTokClient(options.username, options.sessionId, tiktokCookie);

// Listen to stdin for commands
setupStdinListener(client);

// Handle kill signals
process.on('SIGINT', () => client.stop());
process.on('SIGTERM', () => client.stop());

client.connect();
