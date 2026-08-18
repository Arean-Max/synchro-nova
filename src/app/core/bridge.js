export const nativeInvoke =
  window.__TAURI__?.core?.invoke ||
  window.__TAURI__?.tauri?.invoke ||
  null;

export const nativeListen = window.__TAURI__?.event?.listen || null;

export async function invokeCommand(command, payload) {
  if (!nativeInvoke) return null;
  try {
    return await nativeInvoke(command, payload);
  } catch {
    return null;
  }
}
