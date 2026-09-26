// Mock Entra ID for rusteams e2e simulation (Bun, zero dependencies).
// Device-code flow with scripted states; no real credentials, no network
// beyond the compose stack. State lives in memory and resets on restart.

const PORT = Number(process.env.PORT ?? 8080);
const PENDING_POLLS = Number(process.env.PENDING_POLLS ?? 2);

let pollsSeen = 0;
let approved = false;
let mode = "normal"; // normal | slow_down_once | expired

const server = Bun.serve({
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    const parts = url.pathname.split("/").filter(Boolean);

    if (req.method === "GET" && url.pathname === "/health") {
      return Response.json({ ok: true });
    }

    // Test hook: flip mock behaviour without restarting.
    if (req.method === "POST" && url.pathname === "/__mode") {
      const body = await req.json().catch(() => ({}));
      if (typeof body.mode === "string") mode = body.mode;
      if (typeof body.approved === "boolean") approved = body.approved;
      if (typeof body.pendingPolls === "number") {
        pollsSeen = 0;
      }
      return Response.json({ mode, approved, pollsSeen });
    }
    if (req.method === "POST" && url.pathname === "/__reset") {
      pollsSeen = 0;
      approved = true; // default path: user approves after pending polls
      mode = "normal";
      return Response.json({ ok: true });
    }

    // POST /{tenant}/oauth2/v2.0/devicecode
    if (
      req.method === "POST" &&
      parts.length === 4 &&
      parts[1] === "oauth2" &&
      parts[2] === "v2.0" &&
      parts[3] === "devicecode"
    ) {
      console.log(`devicecode tenant=${parts[0]}`);
      pollsSeen = 0;
      return Response.json({
        device_code: "sim-device-code",
        user_code: "SIM-CODE",
        verification_uri: "https://microsoft.com/devicelogin",
        expires_in: 900,
        interval: 1,
        message: "To sign in, enter SIM-CODE at https://microsoft.com/devicelogin (simulated approval, no browser needed)",
      });
    }

    // POST /{tenant}/oauth2/v2.0/token
    if (
      req.method === "POST" &&
      parts.length === 4 &&
      parts[1] === "oauth2" &&
      parts[2] === "v2.0" &&
      parts[3] === "token"
    ) {
      const form = await req.formData().catch(() => null);
      const grant = form?.get("grant_type");
      console.log(`token tenant=${parts[0]} grant=${grant}`);

      if (grant === "refresh_token") {
        return Response.json({
          token_type: "Bearer",
          scope: "openid profile offline_access User.Read",
          expires_in: 3600,
          access_token: "sim-access-token",
          refresh_token: "sim-refresh-token-rotated",
        });
      }

      if (mode === "expired") {
        return Response.json({ error: "expired_token" }, { status: 400 });
      }
      if (mode === "slow_down_once" && pollsSeen === 0) {
        pollsSeen += 1;
        return Response.json({ error: "slow_down" }, { status: 400 });
      }
      if (pollsSeen < PENDING_POLLS || !approved) {
        pollsSeen += 1;
        return Response.json({ error: "authorization_pending" }, { status: 400 });
      }
      return Response.json({
        token_type: "Bearer",
        scope: "openid profile offline_access User.Read",
        expires_in: 3600,
        access_token: "sim-access-token",
        refresh_token: "sim-refresh-token",
      });
    }

    return Response.json({ error: "not found" }, { status: 404 });
  },
});

console.log(`mock-oauth2 listening on :${server.port}`);
