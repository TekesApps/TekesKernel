const state = {
  host: null,
  workspaces: [],
  sessions: [],
  selectedSessionId: null,
  events: [],
  lastSeq: -1,
  historyHasMore: false,
  models: null,
  mux: null,
  muxReady: false,
  journalStreamId: null,
  journalThroughSequence: -1,
  pendingMuxRequests: new Map(),
  actionables: new Map(),
  reconnectAttempt: 0,
  streamGeneration: 0,
  reconnectTimer: null,
  pendingRequest: null,
  pendingPrompts: [],
  inputIsComposing: false,
}

const dom = Object.fromEntries([
  'app-shell', 'workspace-list', 'new-session', 'sidebar-toggle', 'connection-dot',
  'connection-label', 'build-label', 'session-title', 'session-subtitle', 'stop-button',
  'fork-session', 'archive-session',
  'transcript-scroll', 'empty-state', 'transcript', 'request-panel', 'composer',
  'prompt-input', 'send-button', 'model-chip-wrap', 'model-select', 'queue-label',
  'toast-stack',
].map(id => [id.replaceAll('-', '_'), document.getElementById(id)]))

function rpcId(prefix) {
  return `web-${prefix}-${crypto.randomUUID()}`
}

async function rpc(method, payload = {}) {
  const rpcIdValue = rpcId(method.replaceAll('.', '-'))
  const response = await fetch(`/web/api/${encodeURIComponent(method)}`, {
    method: 'POST',
    credentials: 'same-origin',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ type: 'client-request', rpcId: rpcIdValue, method, payload }),
  })
  const value = await response.json().catch(() => null)
  if (!response.ok) throw new Error(value?.error?.message ?? `HTTP ${response.status}`)
  if (value?.type !== 'server-response' || value.rpcId !== rpcIdValue) {
    throw new Error('TekesKernel returned an invalid response envelope')
  }
  if (!value.result?.ok) {
    const error = new Error(value.result?.error?.message ?? 'Request failed')
    error.code = value.result?.error?.code
    error.details = value.result?.error?.details
    throw error
  }
  return value.result.value
}

async function boot() {
  bindEvents()
  setConnection('reconnecting', 'Connecting')
  connectStreams()
}

function bindEvents() {
  dom.sidebar_toggle.addEventListener('click', () => dom.app_shell.classList.toggle('sidebar-open'))
  dom.new_session.addEventListener('click', createSession)
  dom.composer.addEventListener('submit', submitPrompt)
  dom.prompt_input.addEventListener('input', () => {
    resizePrompt()
    dom.send_button.disabled = !state.selectedSessionId || !dom.prompt_input.value.trim()
  })
  dom.prompt_input.addEventListener('compositionstart', () => { state.inputIsComposing = true })
  dom.prompt_input.addEventListener('compositionend', () => { state.inputIsComposing = false })
  dom.prompt_input.addEventListener('keydown', event => {
    if (
      event.key === 'Enter' && !event.shiftKey && !event.isComposing
      && !state.inputIsComposing && event.keyCode !== 229
    ) {
      event.preventDefault()
      dom.composer.requestSubmit()
    }
  })
  dom.stop_button.addEventListener('click', cancelSession)
  dom.fork_session.addEventListener('click', forkSession)
  dom.archive_session.addEventListener('click', archiveSession)
  dom.model_select.addEventListener('change', selectModel)
}

function resizePrompt() {
  dom.prompt_input.style.height = 'auto'
  dom.prompt_input.style.height = `${Math.min(dom.prompt_input.scrollHeight, 180)}px`
}

function setConnection(kind, label) {
  dom.connection_dot.className = `status-dot ${kind}`
  dom.connection_label.textContent = label
}

function renderSidebar() {
  const sessionsByWorkspace = new Map(state.workspaces.map(workspace => [workspace.workspaceId, []]))
  for (const session of state.sessions) {
    const workspace = state.workspaces.find(item => item.sessionIds?.includes(session.sessionId))
    const key = workspace?.workspaceId ?? '__other'
    if (!sessionsByWorkspace.has(key)) sessionsByWorkspace.set(key, [])
    sessionsByWorkspace.get(key).push(session)
  }
  dom.workspace_list.replaceChildren()
  for (const [workspaceId, sessions] of sessionsByWorkspace) {
    if (!sessions.length && workspaceId === '__other') continue
    const workspace = state.workspaces.find(item => item.workspaceId === workspaceId)
    const section = document.createElement('section')
    section.className = 'workspace-section'
    const heading = document.createElement('div')
    heading.className = 'workspace-title'
    const title = document.createElement('span')
    title.textContent = workspace?.title ?? 'Sessions'
    heading.append(title)
    section.append(heading)
    sessions.sort((a, b) => (b.updatedAt ?? 0) - (a.updatedAt ?? 0))
    for (const session of sessions) section.append(sessionButton(session))
    dom.workspace_list.append(section)
  }
  if (!state.sessions.length) {
    const empty = document.createElement('div')
    empty.className = 'sidebar-loading'
    empty.textContent = 'No sessions yet'
    dom.workspace_list.append(empty)
  }
}

function sessionButton(session) {
  const button = document.createElement('button')
  button.type = 'button'
  button.className = `session-row${session.sessionId === state.selectedSessionId ? ' selected' : ''}${session.running ? ' running' : ''}`
  button.dataset.sessionId = session.sessionId
  button.addEventListener('click', () => selectSession(session.sessionId))
  const indicator = document.createElement('span')
  indicator.className = 'session-indicator'
  const copy = document.createElement('span')
  copy.className = 'session-copy'
  const name = document.createElement('span')
  name.className = 'session-name'
  name.textContent = sessionTitle(session)
  const meta = document.createElement('span')
  meta.className = 'session-meta'
  meta.textContent = session.running ? 'Running' : compactPath(session.cwd)
  copy.append(name, meta)
  const time = document.createElement('span')
  time.className = 'session-time'
  time.textContent = relativeTime(session.updatedAt)
  button.append(indicator, copy, time)
  return button
}

function sessionTitle(session) {
  return session.projections?.values?.sessionTitle?.title || (session.blank ? 'New session' : 'Untitled session')
}

function compactPath(path) {
  if (!path) return ''
  const home = state.host?.home
  return home && path.startsWith(home) ? `~${path.slice(home.length)}` : path
}

function relativeTime(timestamp) {
  if (!timestamp) return ''
  const delta = Date.now() - timestamp
  if (delta < 60_000) return 'now'
  if (delta < 3_600_000) return `${Math.floor(delta / 60_000)}m`
  if (delta < 86_400_000) return `${Math.floor(delta / 3_600_000)}h`
  return `${Math.floor(delta / 86_400_000)}d`
}

async function selectSession(sessionId) {
  if (state.selectedSessionId === sessionId && state.events.length) return
  state.selectedSessionId = sessionId
  state.events = []
  state.lastSeq = -1
  state.historyHasMore = false
  state.pendingRequest = null
  selectPendingActionable()
  renderSidebar()
  renderTranscript()
  dom.prompt_input.disabled = true
  dom.send_button.disabled = true
  dom.app_shell.classList.remove('sidebar-open')
  const session = selectedSession()
  updateSessionChrome(session)
  try {
    openJournal(sessionId)
    const models = await rpc('session.models', { sessionId }).catch(() => null)
    if (state.selectedSessionId !== sessionId) return
    state.models = models
    renderModels()
    dom.prompt_input.disabled = false
    dom.send_button.disabled = !dom.prompt_input.value.trim()
    dom.prompt_input.focus()
  } catch (error) {
    if (state.selectedSessionId === sessionId) showToast(error.message, true)
  }
}

function selectedSession() {
  return state.sessions.find(session => session.sessionId === state.selectedSessionId) ?? null
}

function updateSessionChrome(session) {
  dom.session_title.textContent = session ? sessionTitle(session) : 'Select a session'
  dom.session_subtitle.textContent = session ? compactPath(session.cwd) : 'Your native Tekes workspace, in the browser.'
  dom.stop_button.hidden = !session?.running
  dom.fork_session.hidden = !session
  dom.archive_session.hidden = !session
  dom.fork_session.disabled = !session || session.running
  dom.archive_session.disabled = !session || session.running
  dom.queue_label.hidden = !session?.running
}

function commitHistory(history, prepend = false) {
  const incoming = (history.events ?? []).map(row => row.event ?? row)
  const bySeq = new Map((prepend ? [...incoming, ...state.events] : incoming).map(event => [event.seq, event]))
  state.events = [...bySeq.values()].sort((a, b) => a.seq - b.seq)
  state.lastSeq = state.events.at(-1)?.seq ?? -1
  reconcilePendingPrompts(state.events)
  state.historyHasMore = !!history.hasMore
  const title = history.projections?.values?.sessionTitle?.title
  if (title) updateSessionTitle(state.selectedSessionId, title)
  renderTranscript()
}

function renderTranscript() {
  dom.transcript.replaceChildren()
  const rows = [
    ...projectedRows(state.events),
    ...state.pendingPrompts
      .filter(prompt => prompt.sessionId === state.selectedSessionId)
      .map(prompt => ({
        kind: 'message',
        role: 'user',
        content: prompt.content,
        pending: prompt.status,
      })),
  ]
  dom.empty_state.hidden = rows.length > 0 || !!state.selectedSessionId
  if (!state.selectedSessionId) return
  if (!rows.length) {
    dom.empty_state.hidden = false
    dom.empty_state.querySelector('h2').textContent = 'Start a conversation'
    dom.empty_state.querySelector('p').textContent = 'Ask about this workspace, code, or a task you want completed.'
    return
  }
  dom.empty_state.hidden = true
  if (state.historyHasMore) {
    const load = document.createElement('button')
    load.type = 'button'
    load.className = 'load-history'
    load.textContent = 'Load earlier messages'
    load.addEventListener('click', () => loadEarlier(load))
    dom.transcript.append(load)
  }
  for (const row of rows) dom.transcript.append(renderRow(row))
  requestAnimationFrame(() => { dom.transcript_scroll.scrollTop = dom.transcript_scroll.scrollHeight })
}

async function loadEarlier(button) {
  const sessionId = state.selectedSessionId
  const beforeSeq = state.events[0]?.seq
  if (!sessionId || beforeSeq == null) return
  button.disabled = true
  const scrollBottom = dom.transcript_scroll.scrollHeight - dom.transcript_scroll.scrollTop
  try {
    const page = await muxRequest({
      type: 'journal-page', address: { sessionId }, beforeSequence: beforeSeq,
      throughSequence: state.journalThroughSequence, maxMessages: 100,
    })
    if (sessionId !== state.selectedSessionId) return
    commitHistory({ events: page.entries, hasMore: page.hasMoreBefore }, true)
    requestAnimationFrame(() => {
      dom.transcript_scroll.scrollTop = dom.transcript_scroll.scrollHeight - scrollBottom
    })
  } catch (error) {
    button.disabled = false
    showToast(error.message, true)
  }
}

function projectedRows(events) {
  const shadowed = new Set(events.flatMap(event => event.sourceEventSeqs ?? []))
  const rows = []
  const chunks = new Map()
  for (const event of events) {
    if (event.type === 'assistant/chunk' && !shadowed.has(event.seq)) {
      const chunk = event.data?.chunk
      if (chunk?.type === 'text-delta') {
        const key = `${event.data?.turn ?? 0}:${event.data?.step ?? 0}`
        chunks.set(key, (chunks.get(key) ?? '') + chunk.text)
      }
      continue
    }
    if (event.type === 'user/message') rows.push({ kind: 'message', role: 'user', content: event.data?.content ?? [], event })
    if (event.type === 'assistant/message') rows.push({ kind: 'message', role: 'assistant', content: event.data?.message?.content ?? [], event })
    if (event.type === 'tool/call') rows.push({ kind: 'tool', title: event.data?.name ?? 'Tool call', value: event.data })
    if (event.type === 'tool/result') rows.push({ kind: 'tool', title: 'Tool result', value: event.data })
    if (event.type === 'turn/end' && event.data?.reason && !['completed'].includes(event.data.reason)) {
      rows.push({ kind: 'notice', text: `Turn ended: ${event.data.reason}` })
    }
  }
  for (const text of chunks.values()) rows.push({ kind: 'message', role: 'assistant', content: [{ type: 'text', text }], streaming: true })
  return rows
}

function renderRow(row) {
  if (row.kind === 'notice') {
    const notice = document.createElement('div')
    notice.className = 'notice-row'
    notice.textContent = row.text
    return notice
  }
  if (row.kind === 'tool') {
    const details = document.createElement('details')
    details.className = 'tool-card'
    const summary = document.createElement('summary')
    summary.textContent = row.title
    const pre = document.createElement('pre')
    pre.textContent = JSON.stringify(row.value, null, 2)
    details.append(summary, pre)
    return details
  }
  const article = document.createElement('article')
  article.className = `message ${row.role}${row.pending ? ' pending' : ''}`
  const avatar = document.createElement('div')
  avatar.className = 'message-avatar'
  avatar.textContent = row.role === 'user' ? 'You' : 'T'
  const body = document.createElement('div')
  body.className = 'message-body'
  const role = document.createElement('div')
  role.className = 'message-role'
  role.textContent = row.role === 'user' ? 'You' : 'Tekes'
  const content = document.createElement('div')
  content.className = 'message-content'
  renderContent(content, row.content)
  if (row.streaming) {
    const caret = document.createElement('span')
    caret.className = 'streaming-caret'
    content.append(caret)
  }
  body.append(role, content)
  if (row.pending) {
    const meta = document.createElement('div')
    meta.className = 'message-meta'
    meta.textContent = row.pending
    body.append(meta)
  }
  article.append(avatar, body)
  return article
}

function contentText(parts) {
  return (parts ?? [])
    .filter(part => part?.type === 'text')
    .map(part => part.text ?? '')
    .join('')
}

function reconcilePendingPrompts(events) {
  for (const event of events) {
    if (event.type !== 'user/message') continue
    const index = state.pendingPrompts.findIndex(prompt => (
      prompt.sessionId === state.selectedSessionId
      && event.seq > prompt.afterSeq
      && contentText(event.data?.content) === contentText(prompt.content)
    ))
    if (index >= 0) state.pendingPrompts.splice(index, 1)
  }
}

function renderContent(container, parts) {
  for (const part of parts) {
    if (part?.type === 'text') renderText(container, part.text ?? '')
    else if (part?.type === 'tool-call') {
      container.append(renderRow({ kind: 'tool', title: part.name ?? 'Tool call', value: part.arguments ?? part }))
    } else {
      const pre = document.createElement('pre')
      pre.textContent = JSON.stringify(part, null, 2)
      container.append(pre)
    }
  }
}

function renderText(container, text) {
  const blocks = text.split(/(```[\s\S]*?```)/g)
  for (const block of blocks) {
    if (block.startsWith('```') && block.endsWith('```')) {
      const pre = document.createElement('pre')
      const code = document.createElement('code')
      code.textContent = block.replace(/^```[^\n]*\n?/, '').replace(/```$/, '')
      pre.append(code)
      container.append(pre)
      continue
    }
    for (const paragraph of block.split(/\n{2,}/)) {
      if (!paragraph) continue
      const p = document.createElement('p')
      p.textContent = paragraph
      container.append(p)
    }
  }
}

function renderModels() {
  dom.model_select.replaceChildren()
  const models = state.models?.groups?.flatMap(group => group.models.map(model => ({ group, model }))) ?? []
  for (const { group, model } of models) {
    const option = document.createElement('option')
    option.value = JSON.stringify({ provider: group.id, model: model.id })
    option.textContent = `${group.name} · ${model.name}`
    option.selected = state.models.current?.provider === group.id && state.models.current?.model === model.id
    dom.model_select.append(option)
  }
  dom.model_chip_wrap.hidden = models.length === 0
  dom.model_select.disabled = !state.models?.routable || selectedSession()?.running
}

async function selectModel() {
  if (!state.selectedSessionId) return
  const selection = JSON.parse(dom.model_select.value)
  const previous = state.models?.current
  try {
    const value = await rpc('session.selectModel', { sessionId: state.selectedSessionId, ...selection })
    state.models.current = value.selected
  } catch (error) {
    state.models.current = previous
    renderModels()
    showToast(error.message, true)
  }
}

async function submitPrompt(event) {
  event.preventDefault()
  const text = dom.prompt_input.value.trim()
  const sessionId = state.selectedSessionId
  if (!text || !sessionId) return
  dom.prompt_input.value = ''
  dom.prompt_input.style.height = 'auto'
  dom.send_button.disabled = true
  const pending = {
    id: crypto.randomUUID(),
    sessionId,
    afterSeq: state.lastSeq,
    content: [{ type: 'text', text }],
    mode: selectedSession()?.running ? 'steer' : 'queue',
    status: selectedSession()?.running ? 'Steering…' : 'Sending…',
  }
  state.pendingPrompts.push(pending)
  renderTranscript()
  try {
    await rpc('session.prompt', {
      sessionId,
      mode: pending.mode,
      content: [{ type: 'text', text }],
      clientTimeZone: Intl.DateTimeFormat().resolvedOptions().timeZone || 'UTC',
    })
    const admitted = state.pendingPrompts.find(prompt => prompt.id === pending.id)
    if (admitted) {
      admitted.status = pending.mode === 'steer' ? 'Steered' : 'Queued'
      renderTranscript()
    }
  } catch (error) {
    state.pendingPrompts = state.pendingPrompts.filter(prompt => prompt.id !== pending.id)
    dom.prompt_input.value = text
    resizePrompt()
    dom.send_button.disabled = false
    if (error.code === 'steer-unavailable') dom.queue_label.textContent = 'Steering is unavailable; wait or stop the turn'
    showToast(error.message, true)
  }
}

async function cancelSession() {
  if (!state.selectedSessionId) return
  dom.stop_button.disabled = true
  try {
    await rpc('session.cancel', { sessionId: state.selectedSessionId })
  } catch (error) {
    showToast(error.message, true)
  } finally {
    dom.stop_button.disabled = false
  }
}

async function createSession() {
  const workspace = state.workspaces[0]
  if (!workspace) {
    showToast('Create a workspace in Tekes before starting a Web session.', true)
    return
  }
  dom.new_session.disabled = true
  try {
    const value = await rpc('session.create', { workspaceId: workspace.workspaceId })
    await selectSession(value.sessionId)
  } catch (error) {
    showToast(error.message, true)
  } finally {
    dom.new_session.disabled = false
  }
}

async function forkSession() {
  const sessionId = state.selectedSessionId
  if (!sessionId) return
  dom.fork_session.disabled = true
  try {
    const value = await rpc('session.fork', { sessionId })
    await refreshInventory()
    await selectSession(value.sessionId)
  } catch (error) {
    showToast(error.message, true)
  } finally {
    updateSessionChrome(selectedSession())
  }
}

async function archiveSession() {
  const sessionId = state.selectedSessionId
  if (!sessionId) return
  dom.archive_session.disabled = true
  try {
    await rpc('workspace.archiveSession', { sessionId })
    state.selectedSessionId = null
    state.events = []
    state.lastSeq = -1
    await refreshInventory()
    const next = state.sessions[0]
    if (next) await selectSession(next.sessionId)
    else {
      renderTranscript()
      updateSessionChrome(null)
    }
  } catch (error) {
    showToast(error.message, true)
  } finally {
    updateSessionChrome(selectedSession())
  }
}

function connectStreams() {
  const generation = ++state.streamGeneration
  state.muxReady = false
  state.mux?.close()
  state.mux = openSocket('/web/remote.mux', handleRemoteFrame, generation)
}

function openSocket(path, onFrame, generation) {
  const scheme = location.protocol === 'https:' ? 'wss:' : 'ws:'
  const socket = new WebSocket(`${scheme}//${location.host}${path}`)
  socket.addEventListener('message', event => {
    try { onFrame(JSON.parse(event.data)) }
    catch { showToast('TekesKernel sent an invalid V3 frame.', true) }
  })
  socket.addEventListener('close', event => handleSocketClose(generation, event))
  return socket
}

function sendMux(frame) {
  if (!state.mux || state.mux.readyState !== WebSocket.OPEN) {
    throw new Error('Session Endpoint is not connected')
  }
  state.mux.send(JSON.stringify(frame))
}

function openJournal(sessionId) {
  if (!state.muxReady) return
  if (state.journalStreamId) {
    sendMux({ type: 'close', streamId: state.journalStreamId })
  }
  state.journalStreamId = `journal-${crypto.randomUUID()}`
  sendMux({
    type: 'open',
    streamId: state.journalStreamId,
    target: { kind: 'session-journal', address: { sessionId }, maxMessages: 100 },
  })
}

function muxRequest(frame) {
  const requestId = rpcId('mux')
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      state.pendingMuxRequests.delete(requestId)
      reject(new Error('Session Endpoint request timed out'))
    }, 30_000)
    state.pendingMuxRequests.set(requestId, {
      resolve: value => { clearTimeout(timeout); resolve(value) },
      reject: error => { clearTimeout(timeout); reject(error) },
    })
    try { sendMux({ ...frame, requestId }) }
    catch (error) {
      clearTimeout(timeout)
      state.pendingMuxRequests.delete(requestId)
      reject(error)
    }
  })
}

function openBaseStreams() {
  for (const [streamId, kind] of [
    ['workspace', 'workspace'],
    ['inventory', 'session-inventory'],
    ['control', 'session-control'],
    ['actionables', 'actionables'],
  ]) sendMux({ type: 'open', streamId, target: { kind } })
}

function handleRemoteFrame(message) {
  if (message?.type === 'ready') {
    if (message.host?.protocolVersion !== 3) {
      state.mux?.close(1002, 'protocol-version')
      return
    }
    state.muxReady = true
    state.reconnectAttempt = 0
    clearTimeout(state.reconnectTimer)
    state.reconnectTimer = null
    state.host = message.host
    dom.build_label.textContent = message.host.product?.version ?? ''
    setConnection('connected', 'Connected')
    openBaseStreams()
    if (state.selectedSessionId) openJournal(state.selectedSessionId)
    return
  }
  if (message?.type === 'error') {
    const pending = message.requestId && state.pendingMuxRequests.get(message.requestId)
    if (pending) {
      state.pendingMuxRequests.delete(message.requestId)
      pending.reject(new Error(message.error?.message ?? 'Session Endpoint request failed'))
    } else {
      showToast(message.error?.message ?? 'Session Endpoint stream failed', true)
    }
    return
  }
  if (message?.type === 'journal-page-result') {
    const pending = state.pendingMuxRequests.get(message.requestId)
    if (pending) {
      state.pendingMuxRequests.delete(message.requestId)
      pending.resolve(message.page)
    }
    return
  }
  if (message?.type === 'actionable-response-result') {
    const pending = state.pendingMuxRequests.get(message.requestId)
    if (pending) {
      state.pendingMuxRequests.delete(message.requestId)
      pending.resolve(message)
    }
    state.actionables.delete(message.actionableId)
    selectPendingActionable()
    return
  }
  if (message?.type !== 'stream') return
  applySyncFrame(message.streamId, message.frame)
}

function normalizeWorkspace(item) {
  return { ...item, workspaceId: item.id }
}

function applySyncFrame(streamId, frame) {
  if (!frame) return
  if (frame.type === 'workspace-baseline') {
    state.workspaces = (frame.baseline?.items ?? []).map(normalizeWorkspace)
    renderSidebar()
  } else if (frame.type === 'workspace-upsert') {
    const item = normalizeWorkspace(frame.workspace)
    state.workspaces = state.workspaces.filter(row => row.workspaceId !== item.workspaceId)
    state.workspaces.push(item)
    renderSidebar()
  } else if (frame.type === 'workspace-remove') {
    state.workspaces = state.workspaces.filter(row => row.workspaceId !== frame.workspaceId)
    renderSidebar()
  } else if (frame.type === 'workspace-order') {
    const order = new Map(frame.workspaceIds.map((id, index) => [id, index]))
    state.workspaces.sort((a, b) => (order.get(a.workspaceId) ?? 1e9) - (order.get(b.workspaceId) ?? 1e9))
    renderSidebar()
  } else if (frame.type === 'inventory-baseline') {
    state.sessions = frame.items ?? []
    renderSidebar()
    if (!state.selectedSessionId) {
      const first = state.sessions.find(session => !session.blank) ?? state.sessions[0]
      if (first) selectSession(first.sessionId)
    }
  } else if (frame.type === 'inventory-upsert') {
    state.sessions = state.sessions.filter(row => row.sessionId !== frame.session.sessionId)
    state.sessions.push(frame.session)
    renderSidebar()
    updateSessionChrome(selectedSession())
  } else if (frame.type === 'inventory-remove') {
    state.sessions = state.sessions.filter(row => row.sessionId !== frame.sessionId)
    renderSidebar()
  } else if (frame.type === 'journal-snapshot' && streamId === state.journalStreamId) {
    state.journalThroughSequence = frame.snapshot.throughSequence
    commitHistory({
      events: frame.snapshot.entries,
      hasMore: frame.snapshot.hasMoreBefore,
      projections: frame.snapshot.projections,
    })
  } else if (frame.type === 'journal-event' && streamId === state.journalStreamId) {
    handleSessionEvent(frame.address.sessionId, frame.event)
  } else if (frame.type === 'journal-projection' && frame.key === 'sessionTitle') {
    updateSessionTitle(frame.address.sessionId, frame.value?.title)
  } else if (frame.type === 'control-upsert') {
    const session = state.sessions.find(item => item.sessionId === frame.control.sessionId)
    if (session) session.running = !!(frame.control.queue?.length || frame.control.jobs?.length)
    renderSidebar()
    updateSessionChrome(selectedSession())
  } else if (frame.type === 'actionable-baseline') {
    state.actionables = new Map((frame.items ?? []).map(item => [item.id, item]))
    selectPendingActionable()
  } else if (frame.type === 'actionable-upsert') {
    state.actionables.set(frame.actionable.id, frame.actionable)
    selectPendingActionable()
  } else if (frame.type === 'actionable-resolved') {
    state.actionables.delete(frame.id)
    selectPendingActionable()
  }
}

function selectPendingActionable() {
  const actionable = [...state.actionables.values()]
    .find(item => item.sessionId === state.selectedSessionId)
  state.pendingRequest = actionable
    ? { id: actionable.id, revision: actionable.revision, payload: actionable.payload }
    : null
  renderRequest()
}

async function handleSocketClose(generation, event) {
  if (generation !== state.streamGeneration || event.code === 1000) return
  state.muxReady = false
  for (const pending of state.pendingMuxRequests.values()) {
    pending.reject(new Error('Session Endpoint disconnected'))
  }
  state.pendingMuxRequests.clear()
  if (await webSessionExpired()) {
    if (generation !== state.streamGeneration) return
    state.streamGeneration += 1
    clearTimeout(state.reconnectTimer)
    state.reconnectTimer = null
    state.mux?.close()
    setConnection('failed', 'Session expired')
    showToast('Web session expired. Reopen the Web Client from Tekes.', true)
    return
  }
  if (generation !== state.streamGeneration) return
  setConnection('reconnecting', 'Reconnecting')
  if (state.reconnectTimer) return
  const delay = Math.min(1000 * 2 ** state.reconnectAttempt++, 15_000)
  state.reconnectTimer = setTimeout(() => {
    state.reconnectTimer = null
    connectStreams()
  }, delay)
}

async function webSessionExpired() {
  try {
    const response = await fetch('/', { credentials: 'same-origin', cache: 'no-store' })
    return response.status === 401
  } catch {
    return false
  }
}

function refreshInventory() {
  return Promise.resolve()
}

function updateSessionTitle(sessionId, title) {
  if (!title) return
  const session = state.sessions.find(item => item.sessionId === sessionId)
  if (session) {
    session.projections ??= { values: {} }
    session.projections.values ??= {}
    session.projections.values.sessionTitle = { title }
  }
  renderSidebar()
  if (sessionId === state.selectedSessionId) dom.session_title.textContent = title
}

function renderRequest() {
  const pending = state.pendingRequest
  dom.request_panel.replaceChildren()
  dom.request_panel.hidden = !pending || pending.payload.sessionId !== state.selectedSessionId
  if (dom.request_panel.hidden) return
  const title = document.createElement('h2')
  const detail = document.createElement('p')
  const actions = document.createElement('div')
  actions.className = 'request-actions'
  if (pending.payload.type === 'approval/requested') {
    title.textContent = pending.payload.toolName ? `Allow ${pending.payload.toolName}?` : 'Approval required'
    detail.textContent = pending.payload.reason ?? 'Tekes is waiting for permission to continue.'
    actions.append(requestButton('Deny', false), requestButton('Allow once', true, true))
    dom.request_panel.append(title, detail, actions)
  } else {
    title.textContent = 'Tekes needs your input'
    detail.textContent = 'Answer the question to continue the current task.'
    const form = questionForm(pending.payload.questions ?? [])
    dom.request_panel.append(title, detail, form)
  }
}

function requestButton(label, allow, primary = false) {
  const button = document.createElement('button')
  button.type = 'button'
  button.textContent = label
  if (primary) button.className = 'primary'
  button.addEventListener('click', () => respond({
    sessionId: state.pendingRequest.payload.sessionId,
    approvalId: state.pendingRequest.payload.approvalId,
    outcome: allow ? 'allowed-once' : 'rejected',
  }))
  return button
}

function questionForm(questions) {
  const form = document.createElement('form')
  form.className = 'question-form'
  questions.forEach((question, index) => {
    const fieldset = document.createElement('fieldset')
    const legend = document.createElement('legend')
    legend.textContent = question.header ?? question.question
    fieldset.append(legend)
    if (question.header) {
      const prompt = document.createElement('p')
      prompt.textContent = question.question
      fieldset.append(prompt)
    }
    if (question.detail) {
      const detail = document.createElement('small')
      detail.textContent = question.detail
      fieldset.append(detail)
    }
    const options = question.options ?? []
    if (options.length) {
      for (const option of options) {
        const label = document.createElement('label')
        const input = document.createElement('input')
        input.type = question.multiSelect ? 'checkbox' : 'radio'
        input.name = `question-${index}`
        input.value = option.label
        input.required = !question.multiSelect
        const copy = document.createElement('span')
        copy.textContent = option.description ? `${option.label} — ${option.description}` : option.label
        label.append(input, copy)
        fieldset.append(label)
      }
    } else {
      const input = document.createElement('input')
      input.type = 'text'
      input.name = `question-${index}`
      input.required = true
      input.autocomplete = 'off'
      fieldset.append(input)
    }
    form.append(fieldset)
  })
  const submit = document.createElement('button')
  submit.type = 'submit'
  submit.className = 'primary'
  submit.textContent = 'Continue'
  form.append(submit)
  form.addEventListener('submit', event => {
    event.preventDefault()
    const answers = questions.map((question, index) => {
      const inputs = [...form.elements].filter(element => element.name === `question-${index}`)
      if (question.multiSelect) return inputs.filter(input => input.checked).map(input => input.value)
      return inputs.find(input => input.checked)?.value ?? inputs[0]?.value ?? ''
    })
    respond({
      sessionId: state.pendingRequest.payload.sessionId,
      answer: { answers },
    })
  })
  return form
}

async function respond(value) {
  const pending = state.pendingRequest
  if (!pending) return
  try {
    await muxRequest({
      type: 'actionable-respond',
      actionableId: pending.id,
      expectedRevision: pending.revision,
      outcome: value,
    })
  } catch (error) {
    showToast(error.message, true)
  }
}

function showToast(message, error = false) {
  const toast = document.createElement('div')
  toast.className = `toast${error ? ' error' : ''}`
  toast.textContent = message
  dom.toast_stack.append(toast)
  setTimeout(() => toast.remove(), 5000)
}

boot()
