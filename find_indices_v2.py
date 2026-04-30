text = """Comprehensive Analysis of the COSMIC Settings Ecosystem

Executive Summary
The COSMIC Settings application is a state-of-the-art configuration engine designed for the COSMIC desktop environment. It serves as a unified interface for managing everything from low-level hardware drivers to high-level UI aesthetics. This report provides an exhaustive deep-dive into its architecture, functional capabilities, and unique localized implementations.

Advanced Technical Architecture
At its core, COSMIC Settings is a triumph of modern systems programming. Unlike traditional settings apps that rely on heavy web-views or legacy C toolkits, COSMIC Settings is built on a foundation of safety and speed:
- Language: Written entirely in Rust, ensuring memory safety and preventing common runtime crashes.
- GUI Framework: Utilizes the iced library, a cross-platform GUI library for Rust focused on simplicity and type-safety.
- Page Binder System: The application implements a sophisticated "Binder" model (located in cosmic-settings-page). This allows pages to be lazily loaded and cached, significantly reducing the initial memory footprint while maintaining near-instantaneous switching between categories.
- SlotMap Resource Management: Uses the slotmap crate for high-performance, stable indexing of UI sections and settings entities.
- System Integration: Deeply integrated with system daemons like Pipewire (audio), UPower (power), and NetworkManager (connectivity) through asynchronous Rust channels.

ZIQA OS — Localized Visual Identity
A remarkable discovery within the codebase is the specialized integration for ZIQA OS, the "Iraqi Operating System."
- Visual Heritage: The 'About' page features a dedicated "ZIQA Color Palette" inspired by Iraqi culture, utilizing a signature "ZIQA Gold" (#C8A951) and "ZIQA Gold Light" (#FFF3CC).
- Arabic Support: Full Right-to-Left (RTL) support is built into the core layout engine, ensuring a native experience for Arabic speakers.
- Specialized Badges: Hardware categories (CPU, GPU, RAM) are color-coded with the ZIQA palette to provide a cohesive national brand identity within the system settings.

Granular Visual & Theming Details
The app provides unprecedented control over the desktop's look and feel:
- Adaptive Mode Switching: Users can enable "Autoswitch" which uses the ashpd crate to detect daytime/nighttime and automatically transition between Light and Dark modes.
- Interface Density: Offers three distinct density modes: Compact, Standard (Comfortable), and Spacious. This adjustment affects the padding and height of almost every widget in the OS, from the panel to the context menus.
- Corner Radii (Roundness): Three presets—Square, Slightly Round, and Round. These settings are dynamically propagated to the COSMIC Panel and Dock, ensuring that window corners and panel bars match perfectly.
- Experimental Personalization:
    - Font Management: Independent selection for Interface and Monospace fonts with live previews.
    - Icon & Toolkit Styling: Direct control over the Icon Theme and the underlying widget toolkit styles.
    - Window Hinting: A unique "Active Window Hint" that adds a colored border around the currently focused window, with adjustable thickness and color.

Exhaustive Settings Inventory

1. Connectivity — Advanced Networking
- Wireless & Wired: Support for WPA3 security, hidden SSIDs, and enterprise-grade 802.1X authentication.
- VPN Management: Integrated support for OpenVPN and WireGuard with easy-to-use credential management.
- Hotspot: One-click creation of a shared Wi-Fi network with customizable security.

2. Personalization — The Desktop Experience
- Window Management (Granular):
    - Tiling Behavior: Control over how windows auto-tile, including "Focus Follows Cursor" with adjustable millisecond delays.
    - Cursor Behavior: Option for "Cursor Follows Focus" to automatically warp the pointer to a newly focused window.
    - Edge Snap: Adjustable threshold for window snapping to screen edges.
- Workspaces (Multi-Tasking):
    - Action on Typing: Determine what happens when you start typing in the workspace overview (Open Launcher vs. Applications).
    - Workspace Layout: Options for Horizontal vs. Vertical workspace orientations.
    - Wraparound: Enable or disable continuous workspace scrolling.
- Dock & Panel: Precision control over "Expand to Edges," "Anchor Gap," and independent padding for different monitor configurations.

3. Hardware — Low-Level Control
- Power & Battery:
    - Power Profiles: Toggle between Performance, Balanced, and Battery Life modes.
    - Connected Devices: Real-time battery monitoring for Bluetooth peripherals (mice, gamepads, etc.).
    - Idle Timers: Separate "Screen Off" and "Suspend" timers for AC vs. Battery power.
- Sound & Audio:
    - Over-Amplification: Allow volume levels up to 150% for quiet laptops.
    - Device Profiles: Configuration for multi-channel audio cards and HDMI output priorities.
- Display & HIDPI:
    - Fractional Scaling: Support for non-integer scaling (e.g., 125%, 150%) for crisp visuals on 4K screens.

4. System & Global Reach
- Global Language Support: The application is localized into over 70 languages, including Arabic, Persian, Japanese, and various European dialects.
- User Accounts: Detailed management for account types (Standard vs. Administrator), password encryption, and profile picture customization.
- System Diagnostics (About): Provides real-time hostname editing and detailed hardware ID reporting.

Small Details that Matter
- Search Intelligence: A case-insensitive, Unicode-aware search engine that highlights results within specific settings paragraphs.
- Hotkey Access: 'Ctrl + F' globally activates the search bar for rapid navigation.
- Smooth Transitions: The use of the cosmic-iced toolkit ensures that UI transitions (like opening a context drawer) are hardware-accelerated and smooth.

Conclusion
The COSMIC Settings app is more than just a configuration tool; it is a high-performance engine that embodies the future of the Linux desktop. From its use of Rust to its deep integration with the ZIQA OS visual identity and its support for 70+ languages, it sets a new standard for what a system management tool can be. Its modular design ensures that as the COSMIC environment continues to evolve, the settings app will remain its most stable and versatile pillar."""

def find_after(sub, after_idx):
    idx = text.find(sub, after_idx)
    if idx == -1: return -1, -1
    return idx + 1, idx + 1 + len(sub)

# Headers
h1 = find_after("Comprehensive Analysis of the COSMIC Settings Ecosystem", 0)
h2_sections = [
    "Executive Summary", "Advanced Technical Architecture", 
    "ZIQA OS — Localized Visual Identity", "Granular Visual & Theming Details",
    "Exhaustive Settings Inventory", "Small Details that Matter", "Conclusion"
]
h3_sections = [
    "1. Connectivity — Advanced Networking",
    "2. Personalization — The Desktop Experience",
    "3. Hardware — Low-Level Control",
    "4. System & Global Reach"
]

print(f"h1: {h1}")
last = 0
for h in h2_sections:
    s, e = find_after(h, last)
    print(f"h2: ({s}, {e}) # {h}")
    last = e

last = 0
for h in h3_sections:
    s, e = find_after(h, last)
    print(f"h3: ({s}, {e}) # {h}")
    last = e

# Bold list (selection of technical terms)
bold_items = [
    "Language:", "GUI Framework:", "Page Binder System:", "SlotMap Resource Management:", 
    "System Integration:", "Visual Heritage:", "Arabic Support:", "Specialized Badges:",
    "Adaptive Mode Switching:", "Interface Density:", "Corner Radii (Roundness):",
    "Font Management:", "Icon & Toolkit Styling:", "Window Hinting:",
    "Wireless & Wired:", "VPN Management:", "Hotspot:",
    "Window Management (Granular):", "Workspaces (Multi-Tasking):", "Dock & Panel:",
    "Power & Battery:", "Sound & Audio:", "Display & HIDPI:",
    "Global Language Support:", "User Accounts:", "System Diagnostics (About):",
    "Search Intelligence:", "Hotkey Access:", "Smooth Transitions:"
]

for b in bold_items:
    s, e = find_after(b, 0)
    if s != -1:
        print(f"bold: ({s}, {e}) # {b}")
