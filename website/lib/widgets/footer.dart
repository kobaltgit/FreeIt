import 'package:flutter/material.dart';
import 'package:kobalt_ui/kobalt_ui.dart';
import '../theme.dart';
import '../i18n.dart';
import '../project_meta.dart';

class Footer extends StatelessWidget {
  final VoidCallback? onBackToTop;

  const Footer({super.key, this.onBackToTop});

  @override
  Widget build(BuildContext context) {
    return KobaltFooter(
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
      accentColor: AppColors.accent,
      onBackToTop: onBackToTop,
    );
  }
}