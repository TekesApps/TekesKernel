// Official @modelcontextprotocol/sdk servers behind one HTTP port, for the
// Kernel public-tunnel live gates (live-test inventory item B5).
//
//   /tasks/mcp        serverInfo tekes-official-ts-tasks: one tool, slow_echo,
//                     taskSupport "required" through the SDK's experimental
//                     SEP-1686 tasks (tasks/get status, tasks/result payload).
//   /conformance/mcp  serverInfo tekes-official-sdk-conformance: sse_echo over
//                     forced request-scoped SSE (enableJsonResponse=false).
//   POST /conformance/notify  204; asks every connected conformance server to
//                     send notifications/tools/list_changed.
//
// Everything the SDK does not implement in a released version is deliberately
// absent here: the 2026-07-28 modern discovery handshake, x-mcp-header
// parameter headers and multi-round requestState/inputResponses results.
import { createServer } from 'node:http';
import express from 'express';
import { z } from 'zod';
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { StreamableHTTPServerTransport } from '@modelcontextprotocol/sdk/server/streamableHttp.js';
import { InMemoryTaskStore } from '@modelcontextprotocol/sdk/experimental/tasks/stores/in-memory.js';

const port = Number(process.env.PORT || 0);
const taskStore = new InMemoryTaskStore();
const conformanceServers = new Set();

function tasksServer() {
  const server = new McpServer(
    { name: 'tekes-official-ts-tasks', version: '1.0.0' },
    // SEP-1686 server capability: tasks for tools/call, plus list/cancel.
    { capabilities: { tools: {}, tasks: { list: {}, cancel: {}, requests: { tools: { call: {} } } } }, taskStore },
  );
  server.experimental.tasks.registerToolTask(
    'slow_echo',
    {
      description: 'Echoes text after a short delay; always runs as an MCP task.',
      inputSchema: { text: z.string() },
      execution: { taskSupport: 'required' },
    },
    {
      async createTask({ text }, { taskStore: store, taskRequestedTtl }) {
        const task = await store.createTask({ ttl: taskRequestedTtl ?? 60_000, pollInterval: 400 });
        setTimeout(() => {
          store
            .storeTaskResult(task.taskId, 'completed', {
              content: [{ type: 'text', text: `task:${text}` }],
              isError: false,
            })
            .catch(() => {});
        }, 1_200);
        return { task };
      },
      async getTask(_args, { taskId, taskStore: store }) {
        return store.getTask(taskId);
      },
      async getTaskResult(_args, { taskId, taskStore: store }) {
        return store.getTaskResult(taskId);
      },
    },
  );
  return server;
}

function conformanceServer() {
  const server = new McpServer(
    { name: 'tekes-official-sdk-conformance', version: '1.0.0' },
    { capabilities: { tools: { listChanged: true } } },
  );
  server.registerTool(
    'sse_echo',
    { description: 'Echoes a value; the transport forces SSE responses.', inputSchema: { value: z.string() } },
    async ({ value }) => ({ content: [{ type: 'text', text: `sse:${value}` }] }),
  );
  return server;
}

// Stateless transport per request (sessionIdGenerator undefined); every
// response is request-scoped SSE because enableJsonResponse stays false.
function mount(app, path, factory, registry) {
  app.post(path, async (req, res) => {
    const server = factory();
    const transport = new StreamableHTTPServerTransport({ sessionIdGenerator: undefined, enableJsonResponse: false });
    res.on('close', () => {
      registry?.delete(server);
      transport.close().catch(() => {});
      server.close().catch(() => {});
    });
    registry?.add(server);
    await server.connect(transport);
    await transport.handleRequest(req, res, req.body);
  });
  app.get(path, (_req, res) => res.status(405).json({ error: 'stateless: no standalone stream' }));
}

const app = express();
app.use(express.json({ limit: '4mb' }));
mount(app, '/tasks/mcp', tasksServer);
mount(app, '/conformance/mcp', conformanceServer, conformanceServers);
app.post('/conformance/notify', (_req, res) => {
  for (const server of conformanceServers) {
    try { server.sendToolListChanged(); } catch {}
  }
  res.status(204).end();
});
app.get('/healthz', (_req, res) => res.json({ ok: true, sdk: '1.30.0' }));

const http = createServer(app);
http.listen(port, '127.0.0.1', () => {
  const address = http.address();
  process.stdout.write(JSON.stringify({ listening: address.port }) + '\n');
});
