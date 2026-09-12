import 'package:flutter/material.dart';
import 'package:kobalt_ui/kobalt_ui.dart';
import '../theme.dart';
import '../i18n.dart';
import '../project_meta.dart';

class NavBar extends StatelessWidget {
  final VoidCallback onFeaturesTap;
  final VoidCallback onComparisonTap;
  final VoidCallback onFaqTap;
  final VoidCallback onDownloadTap;

  const NavBar({
    super.key,
    required this.onFeaturesTap,
    required this.onComparisonTap,
    required this.onFaqTap,
    required this.onDownloadTap,
  });

  @override
  Widget build(BuildContext context) {
    return KobaltNavBar(
      project: KobaltProjectId.custom,
      customProject: freeItProjectMeta,
      version: 'v1.0.0',
      customLogo: Container(
        width: 36,
        height: 36,
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(10),
          boxShadow: const [
            BoxShadow(
              color: Color(0x4038BDF8),
              blurRadius: 8,
              offset: Offset(0, 2),
            ),
          ],
        ),
        child: ClipRRect(
          borderRadius: BorderRadius.circular(10),
          child: Image.asset('assets/logo.png', width: 36, height: 36, fit: BoxFit.cover),
        ),
      ),
      isRussian: Strings.isRussian,
      onLanguageToggle: toggleLanguage,
      onThemeToggle: toggleSiteTheme,
      isDark: siteThemeMode.value == ThemeMode.dark,
      accentColor: AppColors.accent,
      navLinks: [
        KobaltNavLink(label: Strings.get('Возможности', 'Features'), onTap: onFeaturesTap),
        KobaltNavLink(label: Strings.get('Сравнение', 'Comparison'), onTap: onComparisonTap),
        KobaltNavLink(label: Strings.get('FAQ', 'FAQ'), onTap: onFaqTap),
      ],
      onDownloadTap: onDownloadTap,
    );
  }
}