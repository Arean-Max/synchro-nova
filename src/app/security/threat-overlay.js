export function bindSecurityEvents(nativeListen, t) {
  if (!nativeListen) return;
  nativeListen("security-threat", (event) => showSecurityThreat(event.payload || {}, t));
}

export function showSecurityThreat(threat, t) {
  document.body.classList.add("security-lockdown");
  let overlay = document.querySelector(".security-threat-overlay");
  if (!overlay) {
    overlay = document.createElement("div");
    overlay.className = "security-threat-overlay";
    overlay.setAttribute("role", "alert");
    overlay.setAttribute("aria-live", "assertive");
    overlay.innerHTML = [
      '<div class="security-threat-card">',
      '<div class="security-threat-brand">Synchro Predict</div>',
      '<h2></h2>',
      '<p class="security-threat-detail"></p>',
      '<p class="security-threat-close"></p>',
      "</div>"
    ].join("");
    document.body.appendChild(overlay);
  }

  overlay.querySelector("h2").textContent = threat.title || t("threatDetected");
  overlay.querySelector(".security-threat-detail").textContent = threat.detail || t("shutdownSoon");
  overlay.querySelector(".security-threat-close").textContent = t("shutdownSoon");
}
