text = """COSMIC Settings App: Research & Analysis Report

Introduction
The COSMIC Settings application is the central hub for system configuration in the COSMIC desktop environment, developed by System76. Built using the Rust programming language and the iced GUI library, it leverages the COSMIC Design System to provide a modern, performant, and visually consistent user experience. The app follows a modular architecture, where individual settings pages are registered as independent modules, allowing for easy expansion and maintenance.

Theming and Visual Identity
The visual theme of COSMIC Settings is deeply integrated with the broader COSMIC desktop ecosystem. Key aspects include:

- COSMIC Design System: The app uses a custom set of widgets and styling rules defined in the cosmic-design-system, ensuring that all UI elements follow a unified aesthetic.
- Dark and Light Modes: Full support for system-wide dark and light themes, which can be toggled manually or set to follow a schedule.
- Accent Colors: Users can choose from a variety of accent colors (e.g., blue, green, orange, purple) that are applied consistently across buttons, icons, and active states.
- Navigation Framework: A sidebar-based navigation system categorizes settings into logical groups: Connectivity, Personalization, Hardware, and System. Each category is visually distinguished by unique icons and a consistent color palette.
- Modular Page Binder: The app uses a Binder system to manage the lifecycle and state of each settings page, ensuring that only the active page consumes resources while maintaining a responsive interface.

Settings Inventory

Connectivity
- Networking: Manages wired connections, Wi-Fi networks (including SSID scanning and security protocols), VPN configurations, and Wi-Fi hotspot setup.
- Bluetooth: Facilitates pairing with wireless peripherals like mice, keyboards, and headphones. Includes toggle for Bluetooth radio and visibility settings.

Personalization
- Accessibility: Provides tools for users with diverse needs, including a screen magnifier, high-contrast modes, and specialized keyboard shortcuts.
- Desktop:
    - Appearance: Centralized control for themes (Dark/Light), accent colors, and icon set selection.
    - Wallpaper: Allows users to change the desktop background from a selection of images or custom files.
    - Workspaces: Configuration for workspace behavior, including tiling options and layout management.
    - Dock & Panel: Fine-grained control over the taskbar (Panel) and application launcher (Dock), including size, position, and applet management.
    - Window Management: Controls window tiling behavior, focus rules, and window decoration styles.

Hardware
- Display: Manages monitor resolution, refresh rates, scaling factor (HIDPI support), and multi-monitor layouts.
- Sound: Configuration for audio output and input devices, volume control, and audio profiles (e.g., Stereo, 5.1 Surround).
- Power: Energy management settings, including screen timeout, suspend behavior, and battery optimization for laptops.
- Input:
    - Keyboard: Layout selection, repeat rates, and a comprehensive library of system and custom shortcuts.
    - Mouse & Touchpad: Adjustment for pointer speed, natural scrolling, and click behavior.

System
- Applications: Management for startup applications, default handler assignments for file types, and legacy application support.
- Time & Language: Settings for the system clock, date formatting, and regional/language preferences.
- About: Displays detailed system information, including hardware specifications (CPU, RAM, GPU) and OS version details.
- Updates: Interface for checking and installing system and application updates.
- Users: User account management, including password changes, avatar selection, and account type permissions.

Conclusion
COSMIC Settings represents a significant step forward in Linux desktop configuration. By prioritizing Rust for safety and performance, and the COSMIC Design System for aesthetics, System76 has created a tool that is both powerful for advanced users and accessible for newcomers. Its modular design ensures that as the COSMIC environment grows, the settings app can seamlessly integrate new features while maintaining its core visual identity."""

def find_after(sub, after_idx):
    idx = text.find(sub, after_idx)
    if idx == -1: return -1, -1
    return idx + 1, idx + 1 + len(sub)

inventory_start = text.find("Settings Inventory")

# Major Sections
t_report = find_after("COSMIC Settings App: Research & Analysis Report", 0)
t_intro = find_after("Introduction", 0)
t_theming = find_after("Theming and Visual Identity", 0)
t_inventory = find_after("Settings Inventory", 0)
t_conn = find_after("Connectivity", inventory_start)
t_pers = find_after("Personalization", inventory_start)
t_hard = find_after("Hardware", inventory_start)
t_syst = find_after("System", inventory_start)
t_conc = find_after("Conclusion", inventory_start)

print(f"heading1: {t_report}")
print(f"heading2: {t_intro}, {t_theming}, {t_inventory}, {t_conc}")
print(f"heading3: {t_conn}, {t_pers}, {t_hard}, {t_syst}")

# Bold items
bold_items = [
    "COSMIC Design System:", "Dark and Light Modes:", "Accent Colors:", 
    "Navigation Framework:", "Modular Page Binder:",
    "Networking:", "Bluetooth:", "Accessibility:", "Desktop:",
    "Appearance:", "Wallpaper:", "Workspaces:", "Dock & Panel:", "Window Management:",
    "Display:", "Sound:", "Power:", "Input:", "Keyboard:", "Mouse & Touchpad:",
    "Applications:", "Time & Language:", "About:", "Updates:", "Users:"
]

for item in bold_items:
    s, e = find_after(item, 0)
    print(f"bold: {s}, {e} # {item}")
