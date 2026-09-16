// Official and clean SVG company brand logos for drivers
export function getVendorLogo(vendor, className = "") {
  const v = (vendor || "").toLowerCase();
  const extraClass = className ? ` ${className}` : "";

  switch (v) {
    case "nvidia":
      // Official NVIDIA Eye / Claw polygon badge in GeForce green
      return `<svg class="vendor-logo vendor-nvidia${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M8.9 5.8c-2.4.5-4.5 2.1-5.4 4.3-.4 1-.5 2.5-.2 3.5.7 2.4 2.8 4.2 5.3 4.6 1.4.2 3.1-.1 4.3-.8.4-.2.4-.2.2-.4-.2-.2-.5-.4-.7-.4-.2 0-.6.2-.9.4-1.2.5-2.7.5-3.8.1-1.8-.7-3.1-2.2-3.4-4-.3-1.6.3-3.3 1.6-4.4 1.3-1.1 3-1.4 4.6-1 .6.1 1.5.5 1.7.7.1.1.2 0 .4-.2.2-.2.6-.5.7-.6 0-.1-.6-.4-1.1-.6-.9-.3-2.3-.4-3.3-.2zm2.6 3.1c-1.2.3-2.2 1.1-2.6 2.3-.4 1.1-.1 2.3.7 3.1.8.8 2 1.1 3.1.7.5-.2.9-.5 1.3-.9.1-.1.1-.2 0-.2-.1-.1-.3-.2-.5-.3-.2 0-.4.1-.6.3-.5.4-1.2.5-1.8.4-.9-.2-1.6-.9-1.8-1.7-.2-1 .4-2 1.3-2.4.7-.3 1.6-.3 2.3.1.2.1.4.3.5.4.1.1.1 0 .3-.1.2-.2.5-.4.6-.5 0-.1-.4-.3-.7-.5-.7-.4-1.4-.5-2.1-.4zm.8 2.6c-.3.1-.6.4-.7.7-.2.4-.1.9.2 1.2.3.3.8.4 1.2.2.3-.1.5-.4.6-.7.1-.4 0-.8-.3-1.1-.3-.3-.7-.4-1-.3zm7.6-8.7C17.4 2.2 14.3 2 11.2 2.3c-4.2.5-8 2.9-9.9 6.6C.4 10.7.1 12.8.5 14.9c.7 3.7 3.3 6.9 6.8 8.1 2.4.8 5.2.8 7.6.1 2.1-.6 4-1.7 5.6-3.2.3-.3.2-.4-.1-.6-.3-.2-.6-.5-.9-.7-.1-.1-.2 0-.4.1-1.3 1.2-2.9 2-4.6 2.4-2.5.6-5.1.2-7.2-.9-2.7-1.4-4.5-4-4.9-7-.3-2.3.4-4.6 1.9-6.3 1.8-2.1 4.3-3.2 7-3.1 2.4.1 4.7 1.1 6.3 2.8.5.5 1.1 1.2 1.5 1.8.1.2.2.2.4.1.3-.2.8-.5 1.1-.7.2-.1.1-.2 0-.4-1.3-1.8-3.1-3.3-5.2-4.2z"/>
      </svg>`;

    case "amd":
      // Official AMD Chevron Arrow Badge in Radeon Red
      return `<svg class="vendor-logo vendor-amd${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M18.8 3H5.2C4 3 3 4 3 5.2v13.6C3 20 4 21 5.2 21h13.6c1.2 0 2.2-1 2.2-2.2V5.2C21 4 20 3 18.8 3zM9.8 17.5H6.5v-3.3h3.3v3.3zm0-4.3H6.5V9.9h3.3v3.3zm0-4.3H6.5V6.5h3.3v2.4zm4.3 8.6h-3.3v-3.3h3.3v3.3zm3.4 0h-2.4v-3.3h2.4v3.3zm0-4.3h-2.4V9.9h2.4v3.3zM14.1 9.9h-3.3V6.5h3.3v3.4zm3.4 0h-2.4V6.5h2.4v3.4z"/>
      </svg>`;

    case "intel":
      // Modern Intel Curve Wordmark in Intel Blue
      return `<svg class="vendor-logo vendor-intel${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M7.7 5.2c0-.8-.7-1.4-1.5-1.4-.8 0-1.5.6-1.5 1.4 0 .8.7 1.4 1.5 1.4.8 0 1.5-.6 1.5-1.4zm-2.7 3h2.4V19H5V8.2zm4.3 0h2.3v1.6c.6-1.1 1.8-1.8 3.1-1.8 2.2 0 3.7 1.5 3.7 4.1V19H16v-6.3c0-1.4-.8-2.2-1.9-2.2-1.2 0-2.1.9-2.1 2.3V19H9.3V8.2z"/>
      </svg>`;

    case "realtek":
      // Realtek Crab / Blue Geometric Emblem
      return `<svg class="vendor-logo vendor-realtek${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm3.8 14.8l-3.2-2.1-1.1 1.1v1.7H9.2v-3.7l2.8-2.8-2.8-2.8V4.5h2.3v1.7l1.1 1.1 3.2-2.1 1.2 2-3 2 3 2.4-1.2 2.2z"/>
      </svg>`;

    case "logitech":
      // Logitech "logi" minimalist modern logo
      return `<svg class="vendor-logo vendor-logitech${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M12 3C7 3 3 7 3 12s4 9 9 9 9-4 9-9-4-9-9-9zm-1.5 13.5H8.2V7.5h2.3v9zm5.3 0h-2.3V7.5h2.3v9z"/>
      </svg>`;

    case "qualcomm":
      return `<svg class="vendor-logo vendor-qualcomm${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M12 2C6.5 2 2 6.5 2 12c0 2.6 1 5 2.7 6.8L3 21l3.5-.9C8.2 21.2 10 22 12 22c5.5 0 10-4.5 10-10S17.5 2 12 2zm0 16.5c-3.6 0-6.5-2.9-6.5-6.5S8.4 5.5 12 5.5s6.5 2.9 6.5 6.5-2.9 6.5-6.5 6.5z"/>
      </svg>`;

    case "mediatek":
      return `<svg class="vendor-logo vendor-mediatek${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M4 5h7v7H4V5zm9 0h7v7h-7V5zm0 9h7v5h-7v-5zM4 14h7v5H4v-5z"/>
      </svg>`;

    case "microsoft":
      // Microsoft Windows four-square badge
      return `<svg class="vendor-logo vendor-microsoft${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M3 3h8v8H3V3zm10 0h8v8h-8V3zM3 13h8v8H3v-8zm10 0h8v8h-8v-8z"/>
      </svg>`;

    case "asus":
      return `<svg class="vendor-logo vendor-asus${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M3 8h4l1.5 4h7L17 8h4l-4 9h-3l-1-2.5h-4L8 17H5L3 8z"/>
      </svg>`;

    case "samsung":
      return `<svg class="vendor-logo vendor-samsung${extraClass}" viewBox="0 0 24 24" fill="currentColor">
        <path d="M12 5c-5.5 0-10 3.1-10 7s4.5 7 10 7 10-3.1 10-7-4.5-7-10-7zm3.5 8.2c-.3.4-.8.7-1.4.8-.6.1-1.3.1-2.1.1h-1.8v-4.2h1.7c.8 0 1.5 0 2 .1.6.1 1 .4 1.3.8.3.4.5.9.5 1.2 0 .5-.1.9-.2 1.2z"/>
      </svg>`;

    default:
      // Fallback by device class
      return getClassFallbackLogo(className);
  }
}

export function getClassFallbackLogo(className = "") {
  const c = (className || "").toLowerCase();
  if (c.includes("display") || c.includes("video") || c.includes("gpu")) {
    return `<svg class="vendor-logo vendor-generic display" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
      <rect x="2" y="3" width="20" height="14" rx="2"></rect>
      <line x1="8" y1="21" x2="16" y2="21"></line>
      <line x1="12" y1="17" x2="12" y2="21"></line>
    </svg>`;
  }
  if (c.includes("net") || c.includes("wifi") || c.includes("lan")) {
    return `<svg class="vendor-logo vendor-generic net" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
      <path d="M5 12.55a11 11 0 0 1 14.08 0"></path>
      <path d="M1.42 9a16 16 0 0 1 21.16 0"></path>
      <path d="M8.53 16.11a6 6 0 0 1 6.95 0"></path>
      <line x1="12" y1="20" x2="12.01" y2="20"></line>
    </svg>`;
  }
  if (c.includes("media") || c.includes("audio") || c.includes("sound")) {
    return `<svg class="vendor-logo vendor-generic audio" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
      <path d="M11 5L6 9H2v6h4l5 4V5z"></path>
      <path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
      <path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>
    </svg>`;
  }
  if (c.includes("storage") || c.includes("scsi") || c.includes("disk")) {
    return `<svg class="vendor-logo vendor-generic storage" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
      <rect x="3" y="4" width="18" height="16" rx="2"></rect>
      <line x1="7" y1="8" x2="7.01" y2="8"></line>
      <line x1="7" y1="12" x2="7.01" y2="12"></line>
      <line x1="7" y1="16" x2="17" y2="16"></line>
    </svg>`;
  }
  if (c.includes("bluetooth")) {
    return `<svg class="vendor-logo vendor-generic bluetooth" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
      <path d="m7 7 10 10-5 4V3l5 4L7 17"></path>
    </svg>`;
  }
  if (c.includes("peripheral") || c.includes("mouse") || c.includes("keyboard") || c.includes("hid")) {
    return `<svg class="vendor-logo vendor-generic peripheral" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
      <rect x="5" y="2" width="14" height="20" rx="7"></rect>
      <line x1="12" y1="6" x2="12" y2="10"></line>
    </svg>`;
  }
  // Default Chip / System
  return `<svg class="vendor-logo vendor-generic system" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
    <rect x="8" y="8" width="8" height="8" rx="1"></rect>
    <path d="M4 10h4m-4 4h4m8-4h4m-4 4h4m-6-8v4m4-4v4m-4 8v4m4-4v4"></path>
  </svg>`;
}
