import { contextBridge } from 'electron';

const apiBase = process.env.AIR_SENDER_API_BASE || process.env.AIR_SENDER_CORE_URL || 'http://127.0.0.1:9760';
const apiToken = process.env.AIR_SENDER_API_TOKEN || 'dev-token';

const request = async (method, path, body) => {
  const response = await fetch(`${apiBase}${path}`, {
    method,
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${apiToken}`
    },
    body: body ? JSON.stringify(body) : undefined
  });

  const maybeJson = await response.text();
  const parsed = maybeJson ? JSON.parse(maybeJson) : null;

  if (!response.ok) {
    throw new Error(parsed?.error || `API ${response.status}`);
  }

  return parsed;
};

contextBridge.exposeInMainWorld('receiverApi', {
  getDashboard: () => request('GET', '/v1/dashboard'),
  getProtocols: () => request('GET', '/v1/protocols'),
  updateProtocol: (id, enabled) => request('PATCH', `/v1/protocols/${id}`, { enabled }),
  getSessions: () => request('GET', '/v1/sessions'),
  createSession: (payload) => request('POST', '/v1/sessions', payload),
  acceptSession: (id) => request('POST', `/v1/sessions/${id}/accept`),
  stopSession: (id) => request('POST', `/v1/sessions/${id}/stop`),
  getPolicy: () => request('GET', '/v1/policy'),
  updatePolicy: (payload) => request('PATCH', '/v1/policy', payload),
  getOperatorSettings: () => request('GET', '/v1/operator/settings'),
  updateOperatorSettings: (payload) => request('PATCH', '/v1/operator/settings', payload),
  getConnectInstructions: () => request('GET', '/v1/connect/instructions'),
  getPairingPin: () => request('GET', '/v1/pairing/pin'),
  generatePairingPin: () => request('POST', '/v1/pairing/pin'),
  getTrustedDevices: () => request('GET', '/v1/trust'),
  trustDevice: (id) => request('POST', `/v1/trust/${id}`),
  revokeDevice: (id) => request('DELETE', `/v1/trust/${id}`),
  getRecordings: () => request('GET', '/v1/recordings'),
  startRecording: (payload) => request('POST', '/v1/recordings/start', payload),
  stopRecording: (session_id) => request('POST', '/v1/recordings/stop', { session_id }),
  signConfigProfile: (payload) => request('POST', '/v1/config-profiles/sign', payload),
  verifyConfigProfile: (payload) => request('POST', '/v1/config-profiles/verify', payload),
  getAudit: () => request('GET', '/v1/audit'),
  getPreviewState: () => request('GET', '/v1/preview/state')
});
