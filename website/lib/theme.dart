import 'package:flutter/material.dart';

final ValueNotifier<ThemeMode> siteThemeMode = ValueNotifier<ThemeMode>(ThemeMode.dark);

void toggleSiteTheme() {
  siteThemeMode.value = siteThemeMode.value == ThemeMode.dark ? ThemeMode.light : ThemeMode.dark;
}

class AppColors {
  static bool get isDark => siteThemeMode.value == ThemeMode.dark;

  static Color get background => isDark ? const Color(0xFF0A0E1A) : const Color(0xFFF1F5F9);
  static Color get surface => isDark ? const Color(0xFF141A29) : const Color(0xFFFFFFFF);
  static Color get surfaceCard => isDark ? const Color(0x0AFFFFFF) : const Color(0xFFFFFFFF);
  static Color get surfaceCardHover => isDark ? const Color(0x14FFFFFF) : const Color(0xFFE2E8F0);

  static Color get accent => isDark ? const Color(0xFF38BDF8) : const Color(0xFF0284C7);
  static Color get accentHover => isDark ? const Color(0xFF0284C7) : const Color(0xFF0369A1);

  static Color get textPrimary => isDark ? Colors.white : const Color(0xFF0F172A);
  static Color get textSecondary => isDark ? const Color(0xFF94A3B8) : const Color(0xFF1E293B);
  static Color get textMuted => isDark ? const Color(0xFF64748B) : const Color(0xFF475569);
  static Color get borderSubtle => isDark ? const Color(0x1AFFFFFF) : const Color(0xFFCBD5E1);

  static RadialGradient get heroGlowGradient => RadialGradient(
        center: const Alignment(0.0, -0.6),
        radius: 0.9,
        colors: isDark
            ? const [Color(0x3338BDF8), Color(0x000A0E1A)]
            : const [Color(0x2238BDF8), Color(0x00F1F5F9)],
      );
}

ThemeData buildAppTheme() {
  final isDark = siteThemeMode.value == ThemeMode.dark;
  return (isDark ? ThemeData.dark() : ThemeData.light()).copyWith(
    scaffoldBackgroundColor: AppColors.background,
    colorScheme: (isDark ? const ColorScheme.dark() : const ColorScheme.light()).copyWith(
      primary: AppColors.accent,
      surface: AppColors.surface,
    ),
  );
}
