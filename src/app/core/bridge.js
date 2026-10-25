const getNativeInvoke = () =>
  (typeof window !== "undefined" && (window.__TAURI__?.core?.invoke || window.__TAURI__?.tauri?.invoke)) ||
  null;

export async function invokeCommand(command, payload) {
  const nativeInvoke = getNativeInvoke();
  if (!nativeInvoke) return null;
  try {
    return await nativeInvoke(command, payload);
  } catch (error) {
    console.error(`[nativeInvoke] Command "${command}" failed:`, error);
    throw error;
  }
}
