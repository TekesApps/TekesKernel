// DSH A/B helper: make one line the complete system prompt.
//
// `dsh-system-prompt` registers the deployment persona itself and tool plugins
// add their guidance sections, so a global `dsh-persona` row collides and the
// service cannot be disabled. This row registers a differently named section
// that declares itself complete; assembly then drops every other section.
export const name = 'ab-complete-prompt'
export const inject = ['systemPrompt']
export function apply(ctx, config) {
  ctx.systemPrompt.section({
    name: 'ab:complete-prompt',
    order: 0,
    text: config.text,
    complete: true,
  })
  if (config.suppressRuntimeContext) ctx.systemPrompt.suppressRuntimeContext()
}
