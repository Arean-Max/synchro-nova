export const nativeInvoke =
  window.__TAURI__?.core?.invoke ||
  window.__TAURI__?.tauri?.invoke ||
  null;


export async function invokeCommand(command, payload) {
  if (!nativeInvoke) return null;
  try {
    return await nativeInvoke(command, payload);
  } catch (error) {
    console.warn(`[nativeInvoke] Command "${command}" failed:`, error);
    return null;
  }
}
