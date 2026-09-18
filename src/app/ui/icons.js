const iconPaths = {
  layers: ['<rect x="4" y="4" width="16" height="16" rx="3"></rect>', '<path d="M8 4v4H4"></path>', '<path d="M20 8h-5a3 3 0 0 0-3 3v9"></path>'],
  fileText: ['<path d="M14 2H7a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7z"></path>', '<path d="M14 2v5h5"></path>', '<path d="M9 13h6"></path>', '<path d="M9 17h6"></path>', '<path d="M9 9h1"></path>'],
  monitor: ['<rect x="3" y="4" width="18" height="13" rx="2"></rect>', '<path d="M8 21h8"></path>', '<path d="M12 17v4"></path>'],
  archive: ['<path d="M4 7h16"></path>', '<path d="M5 7l1-4h12l1 4"></path>', '<path d="M6 7v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7"></path>', '<path d="M10 12h4"></path>'],
  grid: ['<rect x="4" y="4" width="6" height="6" rx="1"></rect>', '<rect x="14" y="4" width="6" height="6" rx="1"></rect>', '<rect x="4" y="14" width="6" height="6" rx="1"></rect>', '<rect x="14" y="14" width="6" height="6" rx="1"></rect>'],
  settings: ['<path d="M12 15.5a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7z"></path>', '<path d="M19.4 15a1.7 1.7 0 0 0 .34 1.87l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06A1.7 1.7 0 0 0 15 19.4a1.7 1.7 0 0 0-1 1.55V21a2 2 0 1 1-4 0v-.09A1.7 1.7 0 0 0 9 19.4a1.7 1.7 0 0 0-1.87.34l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.7 1.7 0 0 0 4.6 15a1.7 1.7 0 0 0-1.55-1H3a2 2 0 1 1 0-4h.09A1.7 1.7 0 0 0 4.6 9a1.7 1.7 0 0 0-.34-1.87l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.7 1.7 0 0 0 9 4.6a1.7 1.7 0 0 0 1-1.55V3a2 2 0 1 1 4 0v.09A1.7 1.7 0 0 0 15 4.6a1.7 1.7 0 0 0 1.87-.34l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.7 1.7 0 0 0 19.4 9a1.7 1.7 0 0 0 1.55 1H21a2 2 0 1 1 0 4h-.09A1.7 1.7 0 0 0 19.4 15z"></path>'],
  logOut: ['<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>', '<path d="M16 17l5-5-5-5"></path>', '<path d="M21 12H9"></path>'],
  check: ['<path d="M20 6 9 17l-5-5"></path>'],
  play: ['<path d="M8 5v14l11-7z"></path>'],
  plus: ['<path d="M12 5v14"></path>', '<path d="M5 12h14"></path>'],
  rotate: ['<path d="M3 12a9 9 0 1 0 3-6.7"></path>', '<path d="M3 3v6h6"></path>'],
  save: ['<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>', '<path d="M17 21v-8H7v8"></path>', '<path d="M7 3v5h8"></path>'],
  folder: ['<path d="M3 7a2 2 0 0 1 2-2h5l2 2h7a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>'],
  trash: ['<path d="M3 6h18"></path>', '<path d="M8 6V4h8v2"></path>', '<path d="M19 6l-1 14H6L5 6"></path>', '<path d="M10 11v5"></path>', '<path d="M14 11v5"></path>'],
  shield: ['<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>'],
  chevronDown: ['<path d="m6 9 6 6 6-6"></path>'],
  chevronRight: ['<path d="m9 18 6-6-6-6"></path>'],
  filter: ['<path d="M22 3H2l8 9.5V20l4-2v-5.5z"></path>'],
  gamepad: [
    '<line x1="6" y1="12" x2="10" y2="12"></line>',
    '<line x1="8" y1="10" x2="8" y2="14"></line>',
    '<line x1="15" y1="13" x2="15.01" y2="13"></line>',
    '<line x1="18" y1="11" x2="18.01" y2="11"></line>',
    '<rect x="2" y="6" width="20" height="12" rx="2"></rect>'
  ],
  video: ['<path d="M15 10l4.5-3v10L15 14z"></path>', '<rect x="3" y="6" width="12" height="12" rx="2"></rect>'],
  cpu: ['<rect x="8" y="8" width="8" height="8" rx="1"></rect>', '<path d="M4 10h4"></path>', '<path d="M4 14h4"></path>', '<path d="M16 10h4"></path>', '<path d="M16 14h4"></path>', '<path d="M10 4v4"></path>', '<path d="M14 4v4"></path>', '<path d="M10 16v4"></path>', '<path d="M14 16v4"></path>'],
  bluetooth: ['<path d="m7 7 10 10-5 4V3l5 4L7 17"></path>'],
  volume: ['<path d="M11 5 6 9H3v6h3l5 4z"></path>', '<path d="M15.5 8.5a5 5 0 0 1 0 7"></path>', '<path d="M18.5 5.5a9 9 0 0 1 0 13"></path>'],
  usb: ['<path d="M12 3v12"></path>', '<path d="M8 7l4-4 4 4"></path>', '<path d="M6 13a2 2 0 1 0 0 4 6 6 0 0 0 6-6"></path>', '<path d="M18 13v4"></path>', '<path d="M16 13h4"></path>'],
  network: ['<rect x="3" y="4" width="7" height="6" rx="1"></rect>', '<rect x="14" y="14" width="7" height="6" rx="1"></rect>', '<path d="M7 10v3h10v1"></path>'],
  hardDrive: ['<rect x="3" y="6" width="18" height="12" rx="2"></rect>', '<path d="M7 14h.01"></path>', '<path d="M11 14h6"></path>'],
  mouse: ['<rect x="7" y="3" width="10" height="18" rx="5"></rect>', '<path d="M12 7v4"></path>', '<path d="M12 3v4"></path>'],
  minus: ['<path d="M5 12h14"></path>'],
  square: ['<rect x="6" y="6" width="12" height="12" rx="1"></rect>'],
  x: ['<path d="M18 6 6 18"></path>', '<path d="m6 6 12 12"></path>'],
  sidebar: ['<rect x="3" y="3" width="18" height="18" rx="2"></rect>', '<path d="M9 3v18"></path>'],
  user: ['<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>', '<circle cx="12" cy="7" r="4"></circle>']
};

export function icon(name, extraClass) {
  const paths = iconPaths[name] || iconPaths.layers;
  const className = extraClass ? `icon ${extraClass}` : "icon";
  return `<svg class="${className}" viewBox="0 0 24 24" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">${paths.join("")}</svg>`;
}

export function logo(variant = "mark") {
  const src = variant === "lockup" ? "./assets/synchro-nova-lockup.png" : "./assets/synchro-nova-icon.png";
  const className = variant === "lockup" ? "brand-lockup" : "brand-mark";
  return `<div class="${className}" aria-hidden="true"><img src="${src}" alt=""></div>`;
}
