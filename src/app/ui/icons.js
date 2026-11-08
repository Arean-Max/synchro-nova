const iconPaths = {
  layers: [
    '<polygon points="12 2 2 7 12 12 22 7 12 2"></polygon>',
    '<polyline points="2 17 12 22 22 17"></polyline>',
    '<polyline points="2 12 12 17 22 12"></polyline>'
  ],
  sliders: [
    '<line x1="4" y1="21" x2="4" y2="14"></line>',
    '<line x1="4" y1="10" x2="4" y2="3"></line>',
    '<line x1="12" y1="21" x2="12" y2="12"></line>',
    '<line x1="12" y1="8" x2="12" y2="3"></line>',
    '<line x1="20" y1="21" x2="20" y2="16"></line>',
    '<line x1="20" y1="12" x2="20" y2="3"></line>',
    '<line x1="1" y1="14" x2="7" y2="14"></line>',
    '<line x1="9" y1="8" x2="15" y2="8"></line>',
    '<line x1="17" y1="16" x2="23" y2="16"></line>'
  ],
  tune: [
    '<line x1="4" y1="21" x2="4" y2="14"></line>',
    '<line x1="4" y1="10" x2="4" y2="3"></line>',
    '<line x1="12" y1="21" x2="12" y2="12"></line>',
    '<line x1="12" y1="8" x2="12" y2="3"></line>',
    '<line x1="20" y1="21" x2="20" y2="16"></line>',
    '<line x1="20" y1="12" x2="20" y2="3"></line>',
    '<line x1="1" y1="14" x2="7" y2="14"></line>',
    '<line x1="9" y1="8" x2="15" y2="8"></line>',
    '<line x1="17" y1="16" x2="23" y2="16"></line>'
  ],
  fileText: [
    '<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>',
    '<polyline points="14 2 14 8 20 8"></polyline>',
    '<line x1="16" y1="13" x2="8" y2="13"></line>',
    '<line x1="16" y1="17" x2="8" y2="17"></line>',
    '<polyline points="10 9 9 9 8 9"></polyline>'
  ],
  monitor: [
    '<rect width="20" height="14" x="2" y="3" rx="2"></rect>',
    '<line x1="8" y1="21" x2="16" y2="21"></line>',
    '<line x1="12" y1="17" x2="12" y2="21"></line>'
  ],
  archive: [
    '<rect width="20" height="5" x="2" y="3" rx="2"></rect>',
    '<path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"></path>',
    '<path d="M10 12h4"></path>'
  ],
  settings: [
    '<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path>',
    '<circle cx="12" cy="12" r="3"></circle>'
  ],
  logOut: [
    '<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>',
    '<polyline points="16 17 21 12 16 7"></polyline>',
    '<line x1="21" y1="12" x2="9" y2="12"></line>'
  ],
  download: [
    '<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>',
    '<polyline points="7 10 12 15 17 10"></polyline>',
    '<line x1="12" y1="15" x2="12" y2="3"></line>'
  ],
  check: ['<polyline points="20 6 9 17 4 12"></polyline>'],
  play: ['<polygon points="5 3 19 12 5 21 5 3"></polygon>'],
  rotate: ['<path d="M3 12a9 9 0 1 0 3-6.7"></path>', '<polyline points="3 3 3 9 9 9"></polyline>'],
  save: [
    '<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>',
    '<polyline points="17 21 17 13 7 13 7 21"></polyline>',
    '<polyline points="7 3 7 8 15 8"></polyline>'
  ],
  folder: [
    '<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 8 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"></path>'
  ],
  trash: [
    '<polyline points="3 6 5 6 21 6"></polyline>',
    '<path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>',
    '<line x1="10" y1="11" x2="10" y2="17"></line>',
    '<line x1="14" y1="11" x2="14" y2="17"></line>'
  ],
  shield: ['<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>'],
  shieldCheck: [
    '<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>',
    '<polyline points="9 12 11 14 15 10"></polyline>'
  ],
  gamepad: [
    '<line x1="6" y1="12" x2="10" y2="12"></line>',
    '<line x1="8" y1="10" x2="8" y2="14"></line>',
    '<line x1="15" y1="13" x2="15.01" y2="13"></line>',
    '<line x1="18" y1="11" x2="18.01" y2="11"></line>',
    '<rect width="20" height="12" x="2" y="6" rx="3"></rect>'
  ],
  video: [
    '<polygon points="23 7 16 12 23 17 23 7"></polygon>',
    '<rect width="15" height="14" x="1" y="5" rx="2"></rect>'
  ],
  cpu: [
    '<rect width="14" height="14" x="5" y="5" rx="3"></rect>',
    '<rect width="6" height="6" x="9" y="9" rx="1.5"></rect>',
    '<path d="M9 2v3M15 2v3M9 19v3M15 19v3M2 9h3M2 15h3M19 9h3M19 15h3"></path>'
  ],
  bluetooth: ['<polyline points="7 7 17 17 12 21 12 3 17 7 7 17"></polyline>'],
  volume: [
    '<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>',
    '<path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>',
    '<path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>'
  ],
  network: [
    '<rect width="6" height="6" x="16" y="16" rx="1"></rect>',
    '<rect width="6" height="6" x="2" y="16" rx="1"></rect>',
    '<rect width="6" height="6" x="9" y="2" rx="1"></rect>',
    '<path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3"></path>',
    '<path d="M12 12V8"></path>'
  ],
  hardDrive: [
    '<line x1="22" y1="12" x2="2" y2="12"></line>',
    '<path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path>',
    '<line x1="6" y1="16" x2="6.01" y2="16"></line>',
    '<line x1="10" y1="16" x2="10.01" y2="16"></line>'
  ],
  mouse: [
    '<rect width="12" height="18" x="6" y="3" rx="6"></rect>',
    '<line x1="12" y1="7" x2="12" y2="11"></line>'
  ],
  minus: ['<line x1="5" y1="12" x2="19" y2="12"></line>'],
  plus: ['<line x1="12" y1="5" x2="12" y2="19"></line>', '<line x1="5" y1="12" x2="19" y2="12"></line>'],
  arrowRight: ['<line x1="5" y1="12" x2="19" y2="12"></line>', '<polyline points="12 5 19 12 12 19"></polyline>'],
  chevronDown: ['<polyline points="6 9 12 15 18 9"></polyline>'],
  keyboard: [
    '<rect width="20" height="14" x="2" y="5" rx="3"></rect>',
    '<path d="M6 9h.01M10 9h.01M14 9h.01M18 9h.01M6 13h.01M18 13h.01M10 13h4"></path>'
  ],
  x: ['<path d="M18 6 6 18M6 6l12 12"></path>'],
  sidebar: [
    '<rect width="18" height="18" x="3" y="3" rx="3"></rect>',
    '<path d="M9 3v18"></path>'
  ],
  terminal: [
    '<polyline points="4 17 10 11 4 5"></polyline>',
    '<line x1="12" y1="19" x2="20" y2="19"></line>'
  ],
  alert: [
    '<circle cx="12" cy="12" r="10"></circle>',
    '<line x1="12" y1="8" x2="12" y2="12"></line>',
    '<line x1="12" y1="16" x2="12.01" y2="16"></line>'
  ],
  info: [
    '<circle cx="12" cy="12" r="10"></circle>',
    '<line x1="12" y1="16" x2="12" y2="12"></line>',
    '<line x1="12" y1="8" x2="12.01" y2="8"></line>'
  ],
  help: [
    '<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>',
    '<line x1="12" y1="17" x2="12.01" y2="17"></line>'
  ],
  helpCircle: [
    '<circle cx="12" cy="12" r="10"></circle>',
    '<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>',
    '<line x1="12" y1="17" x2="12.01" y2="17"></line>'
  ],
  crosshair: [
    '<circle cx="12" cy="12" r="10"></circle>',
    '<line x1="22" y1="12" x2="18" y2="12"></line>',
    '<line x1="6" y1="12" x2="2" y2="12"></line>',
    '<line x1="12" y1="6" x2="12" y2="2"></line>',
    '<line x1="12" y1="22" x2="12" y2="18"></line>'
  ],
  moon: ['<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>'],
  eye: [
    '<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path>',
    '<circle cx="12" cy="12" r="3"></circle>'
  ],
  zap: ['<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>'],
  github: [
    '<path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path>',
    '<path d="M9 18c-4.51 2-5-2-7-2"></path>'
  ],
  telegram: [
    '<path d="M21.5 3.5 2.5 10.5l6 2.5 2 6 3.5-3.5 4.5 3.5 3-15.5z"></path>',
    '<path d="M8.5 13 18 6.5l-8 7.5"></path>'
  ],
  snowflake: [
    '<line x1="12" y1="2" x2="12" y2="22"></line>',
    '<line x1="2" y1="12" x2="22" y2="12"></line>',
    '<line x1="20" y1="16" x2="16" y2="12"></line>',
    '<line x1="4" y1="8" x2="8" y2="12"></line>',
    '<line x1="16" y1="4" x2="12" y2="8"></line>',
    '<line x1="8" y1="20" x2="12" y2="16"></line>',
    '<line x1="4" y1="16" x2="8" y2="12"></line>',
    '<line x1="20" y1="8" x2="16" y2="12"></line>',
    '<line x1="8" y1="4" x2="12" y2="8"></line>',
    '<line x1="16" y1="20" x2="12" y2="16"></line>'
  ],
  leaf: [
    '<path d="M11 20A7 7 0 0 1 4 13C4 7 9 3 20 3c0 11-4 16-9 17Z"></path>',
    '<path d="M4 21c4-4 7-9 8-15"></path>'
  ],
  flower: [
    '<circle cx="12" cy="12" r="3"></circle>',
    '<path d="M12 16.5A4.5 4.5 0 1 1 7.5 12 4.5 4.5 0 1 1 12 7.5a4.5 4.5 0 1 1 4.5 4.5 4.5 4.5 0 1 1-4.5 4.5"></path>'
  ],
  sun: [
    '<circle cx="12" cy="12" r="4"></circle>',
    '<path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"></path>'
  ],
  droplet: [
    '<path d="M12 22a7 7 0 0 0 7-7c0-2-1-3.9-3-5.5s-3.5-4-4-6.5c-.5 2.5-2 4.9-4 6.5C6 11.1 5 13 5 15a7 7 0 0 0 7 7z"></path>'
  ],
  sparkles: [
    '<path d="m12 3-1.9 5.8a2 2 0 0 1-1.3 1.3L3 12l5.8 1.9a2 2 0 0 1 1.3 1.3L12 21l1.9-5.8a2 2 0 0 1 1.3-1.3L21 12l-5.8-1.9a2 2 0 0 1-1.3-1.3Z"></path>',
    '<path d="m5 3-1 2.5A1 1 0 0 1 3.5 6L1 7l2.5 1a1 1 0 0 1 .5.5L5 11l1-2.5a1 1 0 0 1 .5-.5L9 7 6.5 6a1 1 0 0 1-.5-.5Z"></path>'
  ]
};

export function icon(name, extraClass) {
  const paths = iconPaths[name] || iconPaths.layers;
  const className = extraClass ? `icon ${extraClass}` : "icon";
  return `<svg class="${className}" viewBox="0 0 24 24" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">${paths.join("")}</svg>`;
}

export function logo(variant = "mark") {
  const src = variant === "lockup" ? "./assets/synchro-nova-lockup.png" : "./assets/synchro-nova-icon.png";
  const className = variant === "lockup" ? "brand-lockup" : "brand-mark";
  return `<div class="${className}" aria-hidden="true"><img src="${src}" alt="Synchro Nova" decoding="async"></div>`;
}
