import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../theme.dart';
import '../i18n.dart';

class DownloadCta extends StatelessWidget {
  const DownloadCta({super.key});

  Future<void> _openUrl(String url) async {
    final uri = Uri.parse(url);
    if (await canLaunchUrl(uri)) {
      await launchUrl(uri, mode: LaunchMode.externalApplication);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 24, vertical: 60),
      padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 48),
      decoration: BoxDecoration(
        color: AppColors.surface,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: AppColors.borderSubtle),
      ),
      child: Column(
        children: [
          Text(
            Strings.get('Попробуйте FreeIt прямо сейчас', 'Try FreeIt Right Now'),
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold, color: AppColors.textPrimary),
          ),
          const SizedBox(height: 12),
          Text(
            Strings.get(
              'Бесплатно, открытый исходный код под лицензией MIT, готов для Windows 10 & 11.',
              'Free, open-source under MIT license, ready for Windows 10 & 11.',
            ),
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 15, color: AppColors.textSecondary),
          ),
          const SizedBox(height: 28),
          MouseRegion(
            cursor: SystemMouseCursors.click,
            child: ElevatedButton.icon(
              style: ElevatedButton.styleFrom(
                backgroundColor: AppColors.accent,
                foregroundColor: Colors.white,
                padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 18),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
              ),
              icon: const Icon(Icons.download),
              label: Text(
                Strings.get('Перейти к релизам на GitHub', 'Go to GitHub Releases'),
                style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
              ),
              onPressed: () => _openUrl('https://github.com/kobaltgit/FreeIt/releases/latest'),
            ),
          ),
        ],
      ),
    );
  }
}